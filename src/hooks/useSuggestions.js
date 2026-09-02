import { useState, useEffect, useMemo, useCallback } from "react";
import { launcherService } from "../services/launcherService";
import { getActiveTokenInfo } from "../utils/commandInput";

function getMatchScore(targetText, query) {
  if (!targetText || !query) return 0;
  const target = targetText.toLowerCase();
  const q = query.toLowerCase();

  if (target === q) return 100; // Exact match
  if (target.startsWith(q)) return 80; // Prefix match
  if (target.includes(`-${q}`) || target.includes(` ${q}`) || target.includes(`/${q}`)) {
    return 60; // Word boundary match
  }
  if (target.includes(q)) return 40; // Substring match
  return 0;
}

const SYSTEM_COMMANDS = [
  {
    id: "sys_add_project",
    kind: "system",
    command: "add-project",
    name: "Add Project",
    description: "Register a new project",
    keywords: ["add", "project", "new", "create"],
  },
  {
    id: "sys_add_app",
    kind: "system",
    command: "add-app",
    name: "Add Application",
    description: "Register a new application",
    keywords: ["add", "app", "application", "new", "create"],
  },
  {
    id: "sys_manage_projects",
    kind: "system",
    command: "manage-projects",
    name: "Manage Projects",
    description: "Browse, edit, and delete projects",
    keywords: ["manage", "projects", "list", "edit", "delete"],
  },
  {
    id: "sys_manage_apps",
    kind: "system",
    command: "manage-apps",
    name: "Manage Applications",
    description: "Browse, edit, and delete applications",
    keywords: ["manage", "apps", "applications", "list", "edit"],
  },
  {
    id: "sys_add_group",
    kind: "system",
    command: "add-group",
    name: "Add Group",
    description: "Create an application group for // launch",
    keywords: ["add", "group", "workspace", "new", "create"],
  },
  {
    id: "sys_manage_groups",
    kind: "system",
    command: "manage-groups",
    name: "Manage Groups",
    description: "Browse and assign default // workspace group",
    keywords: ["manage", "groups", "workspace", "default", "list"],
  },
  {
    id: "sys_help",
    kind: "system",
    command: "help",
    name: "Help & Quickstart",
    description: "View commands, syntax, shortcuts, and guide",
    keywords: ["help", "guide", "docs", "shortcuts", "syntax"],
  },
  {
    id: "sys_settings",
    kind: "system",
    command: "settings",
    name: "Settings",
    description: "Configure autostart, global hotkey, and default group",
    keywords: ["settings", "config", "hotkey", "autostart", "preferences"],
  },
];

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
    const tokenLower = activeToken.toLowerCase().trim();
    const hasInput = input.trim().length > 0;

    // Identify already used application tokens in prefix
    const existingTokens = prefix
      .trim()
      .split(/\s+/)
      .map((t) => t.toLowerCase());

    if (!hasInput) {
      // Empty input -> Helpful top cues
      const initial = [];
      if (hasDefaultGroup) {
        initial.push({
          id: "group_default",
          kind: "group",
          command: "//",
          name: "Default Workspace",
          description: "Launch configured application group",
        });
      }
      for (const app of applications.slice(0, 2)) {
        initial.push({
          id: app.id,
          kind: "app",
          command: app.command,
          name: app.name,
          description: "Open application",
        });
      }
      for (const proj of projects.slice(0, 2)) {
        initial.push({
          id: proj.id,
          kind: "project",
          command: proj.command,
          name: proj.name,
          description: "Open project folder",
        });
      }
      return initial.slice(0, 5);
    }

    const scoredResults = [];

    if (isSlash) {
      // User is typing an application or group slash command
      if ("//".startsWith(tokenLower) && !existingTokens.includes("//")) {
        scoredResults.push({
          id: "group_default",
          kind: "group",
          command: "//",
          name: "Default Workspace",
          description: "Launch default application group",
          score: tokenLower === "//" ? 100 : 90,
        });
      }

      if (
        ("/run".startsWith(tokenLower) || "/r".startsWith(tokenLower)) &&
        !existingTokens.includes("/run") &&
        !existingTokens.includes("/r")
      ) {
        scoredResults.push({
          id: "builtin_run_commands",
          kind: "app",
          command: "/run",
          name: "Run Project Commands",
          description: "Execute configured project run commands / dev servers",
          score: tokenLower === "/run" || tokenLower === "/r" ? 100 : 85,
        });
      }

      for (const app of applications) {
        if (existingTokens.includes(app.command.toLowerCase())) {
          continue; // Skip already specified app
        }

        const cmdScore = getMatchScore(app.command, tokenLower);
        const nameScore = getMatchScore(app.name, tokenLower.replace("/", ""));
        const maxScore = Math.max(cmdScore, nameScore);

        if (maxScore > 0) {
          scoredResults.push({
            id: app.id,
            kind: "app",
            command: app.command,
            name: app.name,
            description: app.projectLaunch?.enabled
              ? "Open in project context"
              : "Launch application",
            score: maxScore,
          });
        }
      }
    } else {
      // User is typing a system command or project name
      for (const sysCmd of SYSTEM_COMMANDS) {
        const cmdScore = getMatchScore(sysCmd.command, tokenLower);
        const nameScore = getMatchScore(sysCmd.name, tokenLower);
        let keywordScore = 0;
        for (const kw of sysCmd.keywords) {
          if (kw.startsWith(tokenLower)) {
            keywordScore = Math.max(keywordScore, 70);
          } else if (kw.includes(tokenLower)) {
            keywordScore = Math.max(keywordScore, 40);
          }
        }

        const maxScore = Math.max(cmdScore, nameScore, keywordScore);
        if (maxScore > 0) {
          scoredResults.push({
            ...sysCmd,
            score: maxScore,
          });
        }
      }

      for (const proj of projects) {
        const cmdScore = getMatchScore(proj.command, tokenLower);
        const nameScore = getMatchScore(proj.name, tokenLower);
        const maxScore = Math.max(cmdScore, nameScore);

        if (maxScore > 0) {
          scoredResults.push({
            id: proj.id,
            kind: "project",
            command: proj.command,
            name: proj.name,
            description: proj.path,
            score: maxScore,
          });
        }
      }
    }

    // Sort descending by score, maintaining stable ordering for ties
    scoredResults.sort((a, b) => b.score - a.score);

    return scoredResults.slice(0, 5);
  }, [input, projects, applications, hasDefaultGroup]);

  return { suggestions, isLoading, reloadSuggestions: loadMetadata };
}
