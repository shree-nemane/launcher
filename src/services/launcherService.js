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
      return await invoke("is_autostart_enabled");
    } catch (e) {
      try {
        return await isEnabled();
      } catch (err) {
        console.warn("Could not query autostart status:", err);
        return false;
      }
    }
  },

  async enableAutostart() {
    try {
      await invoke("enable_autostart");
      return true;
    } catch (e) {
      try {
        await enable();
        return true;
      } catch (err) {
        console.error("Failed to enable autostart:", err);
        throw err;
      }
    }
  },

  async disableAutostart() {
    try {
      await invoke("disable_autostart");
      return false;
    } catch (e) {
      try {
        await disable();
        return false;
      } catch (err) {
        console.error("Failed to disable autostart:", err);
        throw err;
      }
    }
  },

  async pickFolder() {
    return await invoke("pick_folder");
  },

  async pickExecutable() {
    return await invoke("pick_executable");
  },

  async hideLauncher() {
    if (this._isHiding) return;
    this._isHiding = true;
    try {
      await invoke("hide_launcher");
    } catch (e) {
      try {
        const win = getCurrentWindow();
        await win.hide();
      } catch (err) {
        console.warn("Could not hide native window (running outside Tauri?):", err);
      }
    } finally {
      setTimeout(() => {
        this._isHiding = false;
      }, 150);
    }
  },

  async signalLauncherReady() {
    try {
      await invoke("signal_launcher_ready");
    } catch (_) {}
  },
};
