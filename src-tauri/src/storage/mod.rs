use crate::error::AppError;
use crate::models::{Application, ApplicationGroup, Project, Settings};
use crate::validation::{validate_application, validate_group, validate_project};
use chrono::Utc;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
struct CachedItem<T> {
    data: T,
    last_modified: Option<SystemTime>,
}

#[derive(Debug, Clone)]
struct StorageCache {
    projects: CachedItem<Vec<Project>>,
    applications: CachedItem<Vec<Application>>,
    groups: CachedItem<Vec<ApplicationGroup>>,
    settings: CachedItem<Settings>,
}

pub struct StorageManager {
    data_dir: PathBuf,
    cache: RwLock<StorageCache>,
}

impl StorageManager {
    pub fn new(data_dir: PathBuf) -> Result<Self, AppError> {
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir)?;
        }

        Self::ensure_file_exists_static(&data_dir, "projects.json", &Vec::<Project>::new())?;
        Self::ensure_file_exists_static(&data_dir, "applications.json", &Vec::<Application>::new())?;
        Self::ensure_file_exists_static(&data_dir, "groups.json", &Vec::<ApplicationGroup>::new())?;
        Self::ensure_file_exists_static(&data_dir, "settings.json", &Settings::default())?;

        let projects_path = data_dir.join("projects.json");
        let apps_path = data_dir.join("applications.json");
        let groups_path = data_dir.join("groups.json");
        let settings_path = data_dir.join("settings.json");

        let projects: Vec<Project> = Self::read_json_static(&data_dir, "projects.json")?;
        let applications: Vec<Application> = Self::read_json_static(&data_dir, "applications.json")?;
        let groups: Vec<ApplicationGroup> = Self::read_json_static(&data_dir, "groups.json")?;
        let settings: Settings = Self::read_json_static(&data_dir, "settings.json")?;

        let cache = StorageCache {
            projects: CachedItem {
                data: projects,
                last_modified: Self::get_file_last_modified(&projects_path),
            },
            applications: CachedItem {
                data: applications,
                last_modified: Self::get_file_last_modified(&apps_path),
            },
            groups: CachedItem {
                data: groups,
                last_modified: Self::get_file_last_modified(&groups_path),
            },
            settings: CachedItem {
                data: settings,
                last_modified: Self::get_file_last_modified(&settings_path),
            },
        };

        Ok(Self {
            data_dir,
            cache: RwLock::new(cache),
        })
    }

    fn get_file_last_modified(path: &Path) -> Option<SystemTime> {
        fs::metadata(path).and_then(|m| m.modified()).ok()
    }

    fn ensure_file_exists_static<T: serde::Serialize>(data_dir: &Path, filename: &str, default_data: &T) -> Result<(), AppError> {
        let file_path = data_dir.join(filename);
        if !file_path.exists() {
            Self::write_json_atomic_static(&file_path, default_data)?;
        }
        Ok(())
    }

    fn write_json_atomic_static<T: serde::Serialize>(target_path: &Path, data: &T) -> Result<(), AppError> {
        let json_str = serde_json::to_string_pretty(data)?;

        // Before replacing existing valid file, create/update backup of previous state
        if target_path.exists() && target_path.is_file() {
            let backup_path = target_path.with_extension("json.bak");
            let _ = fs::copy(target_path, backup_path);
        }

        let temp_path = target_path.with_extension(format!("tmp_{}", Uuid::new_v4()));

        let mut file = File::create(&temp_path)?;
        file.write_all(json_str.as_bytes())?;
        file.sync_all()?;
        drop(file);

        fs::rename(&temp_path, target_path)?;
        Ok(())
    }

    fn read_json_static<T: serde::de::DeserializeOwned>(data_dir: &Path, filename: &str) -> Result<T, AppError> {
        let file_path = data_dir.join(filename);
        let backup_path = data_dir.join(format!("{}.bak", filename));

        if !file_path.exists() {
            if backup_path.exists() {
                eprintln!(
                    "Warning: Primary file '{}' missing. Attempting recovery from backup...",
                    filename
                );
                let content = fs::read_to_string(&backup_path)?;
                let data = serde_json::from_str(&content)?;
                return Ok(data);
            }
            return Err(AppError::NotFound(format!("Storage file not found: {}", filename)));
        }

        let content = fs::read_to_string(&file_path)?;
        match serde_json::from_str::<T>(&content) {
            Ok(data) => Ok(data),
            Err(primary_err) => {
                eprintln!(
                    "Warning: Failed to parse primary storage file '{}' ({}). Attempting recovery from backup...",
                    filename, primary_err
                );

                if backup_path.exists() {
                    if let Ok(backup_content) = fs::read_to_string(&backup_path) {
                        if let Ok(backup_data) = serde_json::from_str::<T>(&backup_content) {
                            eprintln!("Recovery successful: loaded data from backup for '{}'.", filename);
                            return Ok(backup_data);
                        } else {
                            eprintln!(
                                "Error: Backup file for '{}' is also invalid JSON.",
                                filename
                            );
                        }
                    }
                }

                // Preserve corrupted files on disk without silently overwriting
                Err(AppError::Storage(format!(
                    "Storage file '{}' is corrupted: {}. Corrupted file preserved on disk for recovery.",
                    filename, primary_err
                )))
            }
        }
    }

    fn write_file<T: serde::Serialize>(&self, filename: &str, data: &T) -> Result<(), AppError> {
        let file_path = self.data_dir.join(filename);
        Self::write_json_atomic_static(&file_path, data)
    }

    // ==========================================
    // PROJECTS
    // ==========================================

    pub fn get_projects(&self) -> Result<Vec<Project>, AppError> {
        let file_path = self.data_dir.join("projects.json");
        let disk_mtime = Self::get_file_last_modified(&file_path);

        {
            let guard = self.cache.read().unwrap();
            if disk_mtime.is_some() && guard.projects.last_modified == disk_mtime {
                return Ok(guard.projects.data.clone());
            }
        }

        let mut guard = self.cache.write().unwrap();
        let projects: Vec<Project> = Self::read_json_static(&self.data_dir, "projects.json")?;
        guard.projects = CachedItem {
            data: projects.clone(),
            last_modified: Self::get_file_last_modified(&file_path),
        };
        Ok(projects)
    }

    pub fn get_project(&self, id: &str) -> Result<Project, AppError> {
        let projects = self.get_projects()?;
        projects
            .into_iter()
            .find(|p| p.id == id)
            .ok_or_else(|| AppError::NotFound(format!("Project with ID '{}' not found", id)))
    }

    pub fn find_project_by_command(&self, command: &str) -> Result<Option<Project>, AppError> {
        let projects = self.get_projects()?;
        let cmd_trimmed = command.trim();
        Ok(projects
            .into_iter()
            .find(|p| p.command.trim().eq_ignore_ascii_case(cmd_trimmed)))
    }

    pub fn create_project(&self, mut project: Project) -> Result<Project, AppError> {
        let mut projects = self.get_projects()?;

        if project.id.trim().is_empty() {
            project.id = format!("project_{}", Uuid::new_v4().simple());
        }

        validate_project(&project, &projects, false)?;

        let now = Utc::now().to_rfc3339();
        project.created_at = now.clone();
        project.updated_at = now;

        projects.push(project.clone());
        self.write_file("projects.json", &projects)?;

        let file_path = self.data_dir.join("projects.json");
        let mut guard = self.cache.write().unwrap();
        guard.projects = CachedItem {
            data: projects,
            last_modified: Self::get_file_last_modified(&file_path),
        };

        Ok(project)
    }

    pub fn update_project(&self, mut project: Project) -> Result<Project, AppError> {
        let mut projects = self.get_projects()?;

        let index = projects
            .iter()
            .position(|p| p.id == project.id)
            .ok_or_else(|| AppError::NotFound(format!("Project with ID '{}' not found", project.id)))?;

        validate_project(&project, &projects, true)?;

        project.created_at = projects[index].created_at.clone();
        project.updated_at = Utc::now().to_rfc3339();

        projects[index] = project.clone();
        self.write_file("projects.json", &projects)?;

        let file_path = self.data_dir.join("projects.json");
        let mut guard = self.cache.write().unwrap();
        guard.projects = CachedItem {
            data: projects,
            last_modified: Self::get_file_last_modified(&file_path),
        };

        Ok(project)
    }

    pub fn delete_project(&self, id: &str) -> Result<(), AppError> {
        let mut projects = self.get_projects()?;

        let original_len = projects.len();
        projects.retain(|p| p.id != id);

        if projects.len() == original_len {
            return Err(AppError::NotFound(format!("Project with ID '{}' not found", id)));
        }

        self.write_file("projects.json", &projects)?;

        let file_path = self.data_dir.join("projects.json");
        let mut guard = self.cache.write().unwrap();
        guard.projects = CachedItem {
            data: projects,
            last_modified: Self::get_file_last_modified(&file_path),
        };

        Ok(())
    }

    // ==========================================
    // APPLICATIONS
    // ==========================================

    pub fn get_applications(&self) -> Result<Vec<Application>, AppError> {
        let file_path = self.data_dir.join("applications.json");
        let disk_mtime = Self::get_file_last_modified(&file_path);

        {
            let guard = self.cache.read().unwrap();
            if disk_mtime.is_some() && guard.applications.last_modified == disk_mtime {
                return Ok(guard.applications.data.clone());
            }
        }

        let mut guard = self.cache.write().unwrap();
        let apps: Vec<Application> = Self::read_json_static(&self.data_dir, "applications.json")?;
        guard.applications = CachedItem {
            data: apps.clone(),
            last_modified: Self::get_file_last_modified(&file_path),
        };
        Ok(apps)
    }

    pub fn get_application(&self, id: &str) -> Result<Application, AppError> {
        let apps = self.get_applications()?;
        apps.into_iter()
            .find(|a| a.id == id)
            .ok_or_else(|| AppError::NotFound(format!("Application with ID '{}' not found", id)))
    }

    pub fn find_application_by_command(&self, command: &str) -> Result<Option<Application>, AppError> {
        let apps = self.get_applications()?;
        let cmd_trimmed = command.trim();
        Ok(apps
            .into_iter()
            .find(|a| a.command.trim().eq_ignore_ascii_case(cmd_trimmed)))
    }

    pub fn create_application(&self, mut application: Application) -> Result<Application, AppError> {
        let mut apps = self.get_applications()?;

        if application.id.trim().is_empty() {
            application.id = format!("app_{}", Uuid::new_v4().simple());
        }

        validate_application(&application, &apps, false)?;

        let now = Utc::now().to_rfc3339();
        application.created_at = now.clone();
        application.updated_at = now;

        apps.push(application.clone());
        self.write_file("applications.json", &apps)?;

        let file_path = self.data_dir.join("applications.json");
        let mut guard = self.cache.write().unwrap();
        guard.applications = CachedItem {
            data: apps,
            last_modified: Self::get_file_last_modified(&file_path),
        };

        Ok(application)
    }

    pub fn update_application(&self, mut application: Application) -> Result<Application, AppError> {
        let mut apps = self.get_applications()?;

        let index = apps
            .iter()
            .position(|a| a.id == application.id)
            .ok_or_else(|| AppError::NotFound(format!("Application with ID '{}' not found", application.id)))?;

        validate_application(&application, &apps, true)?;

        application.created_at = apps[index].created_at.clone();
        application.updated_at = Utc::now().to_rfc3339();

        apps[index] = application.clone();
        self.write_file("applications.json", &apps)?;

        let file_path = self.data_dir.join("applications.json");
        let mut guard = self.cache.write().unwrap();
        guard.applications = CachedItem {
            data: apps,
            last_modified: Self::get_file_last_modified(&file_path),
        };

        Ok(application)
    }

    pub fn delete_application(&self, id: &str) -> Result<(), AppError> {
        let mut apps = self.get_applications()?;

        let original_len = apps.len();
        apps.retain(|a| a.id != id);

        if apps.len() == original_len {
            return Err(AppError::NotFound(format!("Application with ID '{}' not found", id)));
        }

        self.write_file("applications.json", &apps)?;

        let app_file_path = self.data_dir.join("applications.json");
        {
            let mut guard = self.cache.write().unwrap();
            guard.applications = CachedItem {
                data: apps,
                last_modified: Self::get_file_last_modified(&app_file_path),
            };
        }

        // Cascading referential cleanup: remove deleted app ID from all groups
        let mut groups = self.get_groups()?;
        let mut groups_modified = false;
        let now = Utc::now().to_rfc3339();

        for group in &mut groups {
            let app_len = group.applications.len();
            let order_len = group.execution_order.len();

            group.applications.retain(|app_id| app_id != id);
            group.execution_order.retain(|app_id| app_id != id);

            if group.applications.len() != app_len || group.execution_order.len() != order_len {
                group.updated_at = now.clone();
                groups_modified = true;
            }
        }

        if groups_modified {
            self.write_file("groups.json", &groups)?;
            let groups_file_path = self.data_dir.join("groups.json");
            let mut guard = self.cache.write().unwrap();
            guard.groups = CachedItem {
                data: groups,
                last_modified: Self::get_file_last_modified(&groups_file_path),
            };
        }

        Ok(())
    }

    // ==========================================
    // APPLICATION GROUPS
    // ==========================================

    pub fn get_groups(&self) -> Result<Vec<ApplicationGroup>, AppError> {
        let file_path = self.data_dir.join("groups.json");
        let disk_mtime = Self::get_file_last_modified(&file_path);

        {
            let guard = self.cache.read().unwrap();
            if disk_mtime.is_some() && guard.groups.last_modified == disk_mtime {
                return Ok(guard.groups.data.clone());
            }
        }

        let mut guard = self.cache.write().unwrap();
        let groups: Vec<ApplicationGroup> = Self::read_json_static(&self.data_dir, "groups.json")?;
        guard.groups = CachedItem {
            data: groups.clone(),
            last_modified: Self::get_file_last_modified(&file_path),
        };
        Ok(groups)
    }

    pub fn get_group(&self, id: &str) -> Result<ApplicationGroup, AppError> {
        let groups = self.get_groups()?;
        groups
            .into_iter()
            .find(|g| g.id == id)
            .ok_or_else(|| AppError::NotFound(format!("Application group with ID '{}' not found", id)))
    }

    pub fn create_group(&self, mut group: ApplicationGroup) -> Result<ApplicationGroup, AppError> {
        let apps = self.get_applications()?;
        let mut groups = self.get_groups()?;

        if group.id.trim().is_empty() {
            group.id = format!("group_{}", Uuid::new_v4().simple());
        }

        validate_group(&group, &apps)?;

        let now = Utc::now().to_rfc3339();
        group.created_at = now.clone();
        group.updated_at = now;

        groups.push(group.clone());
        self.write_file("groups.json", &groups)?;

        let file_path = self.data_dir.join("groups.json");
        let mut guard = self.cache.write().unwrap();
        guard.groups = CachedItem {
            data: groups,
            last_modified: Self::get_file_last_modified(&file_path),
        };

        Ok(group)
    }

    pub fn update_group(&self, mut group: ApplicationGroup) -> Result<ApplicationGroup, AppError> {
        let apps = self.get_applications()?;
        let mut groups = self.get_groups()?;

        let index = groups
            .iter()
            .position(|g| g.id == group.id)
            .ok_or_else(|| AppError::NotFound(format!("Application group with ID '{}' not found", group.id)))?;

        validate_group(&group, &apps)?;

        group.created_at = groups[index].created_at.clone();
        group.updated_at = Utc::now().to_rfc3339();

        groups[index] = group.clone();
        self.write_file("groups.json", &groups)?;

        let file_path = self.data_dir.join("groups.json");
        let mut guard = self.cache.write().unwrap();
        guard.groups = CachedItem {
            data: groups,
            last_modified: Self::get_file_last_modified(&file_path),
        };

        Ok(group)
    }

    pub fn delete_group(&self, id: &str) -> Result<(), AppError> {
        let mut groups = self.get_groups()?;

        let original_len = groups.len();
        groups.retain(|g| g.id != id);

        if groups.len() == original_len {
            return Err(AppError::NotFound(format!("Application group with ID '{}' not found", id)));
        }

        self.write_file("groups.json", &groups)?;

        let groups_file_path = self.data_dir.join("groups.json");
        {
            let mut guard = self.cache.write().unwrap();
            guard.groups = CachedItem {
                data: groups,
                last_modified: Self::get_file_last_modified(&groups_file_path),
            };
        }

        // Cascading referential cleanup: reset defaultApplicationGroupId if this group was default
        let mut settings = self.get_settings()?;
        if settings.default_application_group_id.as_deref() == Some(id) {
            settings.default_application_group_id = None;
            settings.updated_at = Utc::now().to_rfc3339();

            self.write_file("settings.json", &settings)?;
            let settings_file_path = self.data_dir.join("settings.json");
            let mut guard = self.cache.write().unwrap();
            guard.settings = CachedItem {
                data: settings,
                last_modified: Self::get_file_last_modified(&settings_file_path),
            };
        }

        Ok(())
    }

    // ==========================================
    // SETTINGS
    // ==========================================

    pub fn get_settings(&self) -> Result<Settings, AppError> {
        let file_path = self.data_dir.join("settings.json");
        let disk_mtime = Self::get_file_last_modified(&file_path);

        {
            let guard = self.cache.read().unwrap();
            if disk_mtime.is_some() && guard.settings.last_modified == disk_mtime {
                return Ok(guard.settings.data.clone());
            }
        }

        let mut guard = self.cache.write().unwrap();
        let settings: Settings = Self::read_json_static(&self.data_dir, "settings.json")?;
        guard.settings = CachedItem {
            data: settings.clone(),
            last_modified: Self::get_file_last_modified(&file_path),
        };
        Ok(settings)
    }

    pub fn update_settings(&self, mut settings: Settings) -> Result<Settings, AppError> {
        let current_settings = self.get_settings()?;

        settings.created_at = current_settings.created_at;
        settings.updated_at = Utc::now().to_rfc3339();

        self.write_file("settings.json", &settings)?;

        let file_path = self.data_dir.join("settings.json");
        let mut guard = self.cache.write().unwrap();
        guard.settings = CachedItem {
            data: settings.clone(),
            last_modified: Self::get_file_last_modified(&file_path),
        };

        Ok(settings)
    }
}
