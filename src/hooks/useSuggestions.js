import { useState, useEffect, useMemo, useCallback } from "react";
import { launcherService } from "../services/launcherService";
import { getActiveTokenInfo } from "../utils/commandInput";

export function useSuggestions(input) {
  const [projects, setProjects] = useState([]);
  const [applications, setApplications] = useState([]);
  const [hasDefaultGroup, setHasDefaultGroup] = useState(false);
  const [isLoading, setIsLoading] = useState(true);

  const loadMetadata = useCallback(async () => {
    try {
      const [projList, appList, settings] = await Promise.all([
        launcherService.getProjects(),
        launcherService.getApplications(),
        launcherService.getSettings(),
      ]);
      setProjects(projList || []);
      setApplications(appList || []);
      setHasDefaultGroup(Boolean(settings?.defaultApplicationGroupId));
    } catch (err) {
      console.error("Failed to load launcher suggestions metadata:", err);
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    loadMetadata();
  }, [loadMetadata]);

  const suggestions = useMemo(() => {
    const { activeToken, prefix, isSlash } = getActiveTokenInfo(input);
    const tokenLower = activeToken.toLowerCase();
    const hasInput = input.trim().length > 0;

    // Identify already used application tokens in prefix
    const existingTokens = prefix
      .trim()
      .split(/\s+/)
      .map((t) => t.toLowerCase());

    let results = [];

    if (!hasInput) {
      // Empty input -> Helpful top cues
      if (hasDefaultGroup) {
        results.push({
          id: "group_default",
          kind: "group",
          command: "//",
          name: "Default Workspace",
          description: "Launch configured application group",
        });
      }
      for (const app of applications.slice(0, 2)) {
        results.push({
          id: app.id,
          kind: "app",
          command: app.command,
          name: app.name,
          description: "Open application",
        });
      }
      for (const proj of projects.slice(0, 2)) {
        results.push({
          id: proj.id,
          kind: "project",
          command: proj.command,
          name: proj.name,
          description: "Open project folder",
        });
      }
      return results.slice(0, 5);
    }

    if (isSlash) {
      // User is typing an application or group slash command
      if ("//".startsWith(tokenLower) && !existingTokens.includes("//")) {
        results.push({
          id: "group_default",
          kind: "group",
          command: "//",
          name: "Default Workspace",
          description: "Launch default application group",
        });
      }

      if (
        ("/run".startsWith(tokenLower) || "/r".startsWith(tokenLower)) &&
        !existingTokens.includes("/run") &&
        !existingTokens.includes("/r")
      ) {
        results.push({
          id: "builtin_run_commands",
          kind: "app",
          command: "/run",
          name: "Run Project Commands",
          description: "Execute configured project run commands / dev servers",
        });
      }

      for (const app of applications) {
        if (existingTokens.includes(app.command.toLowerCase())) {
          continue; // Skip already specified app
        }

        const cmdMatch = app.command.toLowerCase().includes(tokenLower);
        const nameMatch = app.name
          .toLowerCase()
          .includes(tokenLower.replace("/", ""));

        if (cmdMatch || nameMatch) {
          results.push({
            id: app.id,
            kind: "app",
            command: app.command,
            name: app.name,
            description: app.projectLaunch?.enabled
              ? "Open in project context"
              : "Launch application",
          });
        }
      }
    } else {
      // User is typing a project name or system command
      if ("add-project".startsWith(tokenLower)) {
        results.push({
          id: "sys_add_project",
          kind: "system",
          command: "add-project",
          name: "Add Project",
          description: "Register a new project",
        });
      }
      if ("add-app".startsWith(tokenLower)) {
        results.push({
          id: "sys_add_app",
          kind: "system",
          command: "add-app",
          name: "Add Application",
          description: "Register a new application",
        });
      }
      if ("manage-projects".startsWith(tokenLower)) {
        results.push({
          id: "sys_manage_projects",
          kind: "system",
          command: "manage-projects",
          name: "Manage Projects",
          description: "Browse, edit, and delete projects",
        });
      }
      if ("manage-apps".startsWith(tokenLower)) {
        results.push({
          id: "sys_manage_apps",
          kind: "system",
          command: "manage-apps",
          name: "Manage Applications",
          description: "Browse, edit, and delete applications",
        });
      }
      if ("add-group".startsWith(tokenLower)) {
        results.push({
          id: "sys_add_group",
          kind: "system",
          command: "add-group",
          name: "Add Group",
          description: "Create an application group for // launch",
        });
      }
      if ("manage-groups".startsWith(tokenLower)) {
        results.push({
          id: "sys_manage_groups",
          kind: "system",
          command: "manage-groups",
          name: "Manage Groups",
          description: "Browse and assign default // workspace group",
        });
      }
      if ("help".startsWith(tokenLower)) {
        results.push({
          id: "sys_help",
          kind: "system",
          command: "help",
          name: "Help & Quickstart",
          description: "View commands, syntax, shortcuts, and guide",
        });
      }
      if ("settings".startsWith(tokenLower) || "config".startsWith(tokenLower)) {
        results.push({
          id: "sys_settings",
          kind: "system",
          command: "settings",
          name: "Settings",
          description: "Configure autostart, global hotkey, and default group",
        });
      }

      for (const proj of projects) {
        const cmdMatch = proj.command.toLowerCase().includes(tokenLower);
        const nameMatch = proj.name.toLowerCase().includes(tokenLower);
        if (cmdMatch || nameMatch) {
          results.push({
            id: proj.id,
            kind: "project",
            command: proj.command,
            name: proj.name,
            description: proj.path,
          });
        }
      }
    }

    return results.slice(0, 5);
  }, [input, projects, applications, hasDefaultGroup]);

  return { suggestions, isLoading, reloadSuggestions: loadMetadata };
}
