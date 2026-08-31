import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { enable, disable, isEnabled } from "@tauri-apps/plugin-autostart";

/**
 * Service encapsulating Tauri IPC commands.
 */
export const launcherService = {
  async planCommand(input) {
    return await invoke("plan_command", { input });
  },

  async executeCommand(input) {
    return await invoke("execute_command", { input });
  },

  async getProjects() {
    return await invoke("get_projects");
  },

  async getProject(id) {
    return await invoke("get_project", { id });
  },

  async createProject(project) {
    return await invoke("create_project", { project });
  },

  async updateProject(project) {
    return await invoke("update_project", { project });
  },

  async deleteProject(id) {
    return await invoke("delete_project", { id });
  },

  async getApplications() {
    return await invoke("get_applications");
  },

  async getApplication(id) {
    return await invoke("get_application", { id });
  },

  async createApplication(application) {
    return await invoke("create_application", { application });
  },

  async updateApplication(application) {
    return await invoke("update_application", { application });
  },

  async deleteApplication(id) {
    return await invoke("delete_application", { id });
  },

  async getGroups() {
    return await invoke("get_groups");
  },

  async getGroup(id) {
    return await invoke("get_group", { id });
  },

  async createGroup(group) {
    return await invoke("create_group", { group });
  },

  async updateGroup(group) {
    return await invoke("update_group", { group });
  },

  async deleteGroup(id) {
    return await invoke("delete_group", { id });
  },

  async getSettings() {
    return await invoke("get_settings");
  },

  async updateSettings(settings) {
    return await invoke("update_settings", { settings });
  },

  async setDefaultGroup(groupId) {
    const current = await invoke("get_settings");
    return await invoke("update_settings", {
      settings: {
        ...current,
        defaultApplicationGroupId: groupId,
      },
    });
  },

  async isAutostartEnabled() {
    try {
      return await isEnabled();
    } catch (e) {
      console.warn("Could not query autostart status:", e);
      return false;
    }
  },

  async enableAutostart() {
    try {
      await enable();
      return true;
    } catch (e) {
      console.error("Failed to enable autostart:", e);
      throw e;
    }
  },

  async disableAutostart() {
    try {
      await disable();
      return false;
    } catch (e) {
      console.error("Failed to disable autostart:", e);
      throw e;
    }
  },

  async pickFolder() {
    return await invoke("pick_folder");
  },

  async pickExecutable() {
    return await invoke("pick_executable");
  },

  async hideLauncher() {
    try {
      await invoke("hide_launcher");
    } catch (e) {
      try {
        const win = getCurrentWindow();
        await win.hide();
      } catch (err) {
        console.warn("Could not hide native window (running outside Tauri?):", err);
      }
    }
  },
};
