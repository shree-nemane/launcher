use crate::error::AppError;
use crate::models::{Application, ApplicationGroup, Project, Settings};
use crate::validation::{validate_application, validate_group, validate_project};
use chrono::Utc;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use uuid::Uuid;

pub struct StorageManager {
    data_dir: PathBuf,
    lock: RwLock<()>,
}

impl StorageManager {
    pub fn new(data_dir: PathBuf) -> Result<Self, AppError> {
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir)?;
        }

        let manager = Self {
            data_dir,
            lock: RwLock::new(()),
        };

        manager.ensure_file_exists("projects.json", &Vec::<Project>::new())?;
        manager.ensure_file_exists("applications.json", &Vec::<Application>::new())?;
        manager.ensure_file_exists("groups.json", &Vec::<ApplicationGroup>::new())?;
        manager.ensure_file_exists("settings.json", &Settings::default())?;

        Ok(manager)
    }

    fn ensure_file_exists<T: serde::Serialize>(&self, filename: &str, default_data: &T) -> Result<(), AppError> {
        let file_path = self.data_dir.join(filename);
        if !file_path.exists() {
            self.write_json_atomic(&file_path, default_data)?;
        }
        Ok(())
    }

    fn write_json_atomic<T: serde::Serialize>(&self, target_path: &Path, data: &T) -> Result<(), AppError> {
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

    fn read_json<T: serde::de::DeserializeOwned>(&self, filename: &str) -> Result<T, AppError> {
        let file_path = self.data_dir.join(filename);
        let backup_path = self.data_dir.join(format!("{}.bak", filename));

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
        self.write_json_atomic(&file_path, data)
    }

    // ==========================================
    // PROJECTS
    // ==========================================

    pub fn get_projects(&self) -> Result<Vec<Project>, AppError> {
        let _guard = self.lock.read().unwrap();
        self.read_json("projects.json")
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
        let _guard = self.lock.write().unwrap();
        let mut projects: Vec<Project> = self.read_json("projects.json")?;

        if project.id.trim().is_empty() {
            project.id = format!("project_{}", Uuid::new_v4().simple());
        }

        validate_project(&project, &projects, false)?;

        let now = Utc::now().to_rfc3339();
        project.created_at = now.clone();
        project.updated_at = now;

        projects.push(project.clone());
        self.write_file("projects.json", &projects)?;
        Ok(project)
    }

    pub fn update_project(&self, mut project: Project) -> Result<Project, AppError> {
        let _guard = self.lock.write().unwrap();
        let mut projects: Vec<Project> = self.read_json("projects.json")?;

        let index = projects
            .iter()
            .position(|p| p.id == project.id)
            .ok_or_else(|| AppError::NotFound(format!("Project with ID '{}' not found", project.id)))?;

        validate_project(&project, &projects, true)?;

        project.created_at = projects[index].created_at.clone();
        project.updated_at = Utc::now().to_rfc3339();

        projects[index] = project.clone();
        self.write_file("projects.json", &projects)?;
        Ok(project)
    }

    pub fn delete_project(&self, id: &str) -> Result<(), AppError> {
        let _guard = self.lock.write().unwrap();
        let mut projects: Vec<Project> = self.read_json("projects.json")?;

        let original_len = projects.len();
        projects.retain(|p| p.id != id);

        if projects.len() == original_len {
            return Err(AppError::NotFound(format!("Project with ID '{}' not found", id)));
        }

        self.write_file("projects.json", &projects)?;
        Ok(())
    }

    // ==========================================
    // APPLICATIONS
    // ==========================================

    pub fn get_applications(&self) -> Result<Vec<Application>, AppError> {
        let _guard = self.lock.read().unwrap();
        self.read_json("applications.json")
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
        let _guard = self.lock.write().unwrap();
        let mut apps: Vec<Application> = self.read_json("applications.json")?;

        if application.id.trim().is_empty() {
            application.id = format!("app_{}", Uuid::new_v4().simple());
        }

        validate_application(&application, &apps, false)?;

        let now = Utc::now().to_rfc3339();
        application.created_at = now.clone();
        application.updated_at = now;

        apps.push(application.clone());
        self.write_file("applications.json", &apps)?;
        Ok(application)
    }

    pub fn update_application(&self, mut application: Application) -> Result<Application, AppError> {
        let _guard = self.lock.write().unwrap();
        let mut apps: Vec<Application> = self.read_json("applications.json")?;

        let index = apps
            .iter()
            .position(|a| a.id == application.id)
            .ok_or_else(|| AppError::NotFound(format!("Application with ID '{}' not found", application.id)))?;

        validate_application(&application, &apps, true)?;

        application.created_at = apps[index].created_at.clone();
        application.updated_at = Utc::now().to_rfc3339();

        apps[index] = application.clone();
        self.write_file("applications.json", &apps)?;
        Ok(application)
    }

    pub fn delete_application(&self, id: &str) -> Result<(), AppError> {
        let _guard = self.lock.write().unwrap();
        let mut apps: Vec<Application> = self.read_json("applications.json")?;

        let original_len = apps.len();
        apps.retain(|a| a.id != id);

        if apps.len() == original_len {
            return Err(AppError::NotFound(format!("Application with ID '{}' not found", id)));
        }

        self.write_file("applications.json", &apps)?;

        // Cascading referential cleanup: remove deleted app ID from all groups
        let mut groups: Vec<ApplicationGroup> = self.read_json("groups.json")?;
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
        }

        Ok(())
    }

    // ==========================================
    // APPLICATION GROUPS
    // ==========================================

    pub fn get_groups(&self) -> Result<Vec<ApplicationGroup>, AppError> {
        let _guard = self.lock.read().unwrap();
        self.read_json("groups.json")
    }

    pub fn get_group(&self, id: &str) -> Result<ApplicationGroup, AppError> {
        let groups = self.get_groups()?;
        groups
            .into_iter()
            .find(|g| g.id == id)
            .ok_or_else(|| AppError::NotFound(format!("Application group with ID '{}' not found", id)))
    }

    pub fn create_group(&self, mut group: ApplicationGroup) -> Result<ApplicationGroup, AppError> {
        let _guard = self.lock.write().unwrap();
        let apps: Vec<Application> = self.read_json("applications.json")?;
        let mut groups: Vec<ApplicationGroup> = self.read_json("groups.json")?;

        if group.id.trim().is_empty() {
            group.id = format!("group_{}", Uuid::new_v4().simple());
        }

        validate_group(&group, &apps)?;

        let now = Utc::now().to_rfc3339();
        group.created_at = now.clone();
        group.updated_at = now;

        groups.push(group.clone());
        self.write_file("groups.json", &groups)?;
        Ok(group)
    }

    pub fn update_group(&self, mut group: ApplicationGroup) -> Result<ApplicationGroup, AppError> {
        let _guard = self.lock.write().unwrap();
        let apps: Vec<Application> = self.read_json("applications.json")?;
        let mut groups: Vec<ApplicationGroup> = self.read_json("groups.json")?;

        let index = groups
            .iter()
            .position(|g| g.id == group.id)
            .ok_or_else(|| AppError::NotFound(format!("Application group with ID '{}' not found", group.id)))?;

        validate_group(&group, &apps)?;

        group.created_at = groups[index].created_at.clone();
        group.updated_at = Utc::now().to_rfc3339();

        groups[index] = group.clone();
        self.write_file("groups.json", &groups)?;
        Ok(group)
    }

    pub fn delete_group(&self, id: &str) -> Result<(), AppError> {
        let _guard = self.lock.write().unwrap();
        let mut groups: Vec<ApplicationGroup> = self.read_json("groups.json")?;

        let original_len = groups.len();
        groups.retain(|g| g.id != id);

        if groups.len() == original_len {
            return Err(AppError::NotFound(format!("Application group with ID '{}' not found", id)));
        }

        self.write_file("groups.json", &groups)?;

        // Cascading referential cleanup: reset defaultApplicationGroupId if this group was default
        let mut settings: Settings = self.read_json("settings.json")?;
        if settings.default_application_group_id.as_deref() == Some(id) {
            settings.default_application_group_id = None;
            settings.updated_at = Utc::now().to_rfc3339();
            self.write_file("settings.json", &settings)?;
        }

        Ok(())
    }

    // ==========================================
    // SETTINGS
    // ==========================================

    pub fn get_settings(&self) -> Result<Settings, AppError> {
        let _guard = self.lock.read().unwrap();
        self.read_json("settings.json")
    }

    pub fn update_settings(&self, mut settings: Settings) -> Result<Settings, AppError> {
        let _guard = self.lock.write().unwrap();
        let current_settings: Settings = self.read_json("settings.json")?;

        settings.created_at = current_settings.created_at;
        settings.updated_at = Utc::now().to_rfc3339();

        self.write_file("settings.json", &settings)?;
        Ok(settings)
    }
}
