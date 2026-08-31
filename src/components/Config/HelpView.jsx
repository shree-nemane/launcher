import React, { useState } from "react";
import { CloseIcon, SearchIcon, FolderIcon, AppIcon, GroupIcon, HelpIcon } from "../Launcher/Icons";

export function HelpView({ onCancel }) {
  const [activeTab, setActiveTab] = useState("all");
  const [searchFilter, setSearchFilter] = useState("");

  const handleKeyDown = (e) => {
    if (e.key === "Escape") {
      e.preventDefault();
      onCancel();
    }
  };

  const sections = [
    {
      id: "quickstart",
      category: "syntax",
      title: "Core Launch Syntax",
      items: [
        {
          syntax: "<project>",
          example: "goofies",
          description: "Opens the project folder in Windows File Explorer.",
          tag: "Explorer",
          tagColor: "bg-amber-950/60 border-amber-800/80 text-amber-300",
        },
        {
          syntax: "<project> /run",
          example: "goofies /run",
          description: "Spawns all configured Run Commands (e.g. npm run dev) in PowerShell 7 (pwsh).",
          tag: "PowerShell 7",
          tagColor: "bg-blue-950/60 border-blue-800/80 text-blue-300",
        },
        {
          syntax: "<project> /<app>",
          example: "goofies /v",
          description: "Opens the project inside the specified application (e.g., VS Code).",
          tag: "App Context",
          tagColor: "bg-indigo-950/60 border-indigo-800/80 text-indigo-300",
        },
        {
          syntax: "<project> /v /run",
          example: "goofies /v /run",
          description: "Launches VS Code AND starts the dev server in PowerShell 7 simultaneously.",
          tag: "Multi-Action",
          tagColor: "bg-emerald-950/60 border-emerald-800/80 text-emerald-300",
        },
        {
          syntax: "<project> //",
          example: "goofies //",
          description: "Opens the project across all applications in your Default Development Group.",
          tag: "Default Group",
          tagColor: "bg-purple-950/60 border-purple-800/80 text-purple-300",
        },
        {
          syntax: "//",
          example: "//",
          description: "Launches all tools in your Default Workspace Group standalone without project context.",
          tag: "Standalone Group",
          tagColor: "bg-purple-950/60 border-purple-800/80 text-purple-300",
        },
      ],
    },
    {
      id: "management",
      category: "management",
      title: "System Management Commands",
      items: [
        {
          syntax: "add-project",
          example: "add-project",
          description: "Register a new project with directory, URL, and Run Commands (npm run dev, python, etc.).",
          tag: "Projects",
          tagColor: "bg-amber-950/60 border-amber-800/80 text-amber-300",
        },
        {
          syntax: "manage-projects",
          example: "manage-projects",
          description: "Browse, filter, edit settings, or remove registered projects.",
          tag: "Projects",
          tagColor: "bg-amber-950/60 border-amber-800/80 text-amber-300",
        },
        {
          syntax: "add-app",
          example: "add-app",
          description: "Register an application (VS Code, PowerShell, Terminal, Browser) with custom arguments.",
          tag: "Apps",
          tagColor: "bg-blue-950/60 border-blue-800/80 text-blue-300",
        },
        {
          syntax: "manage-apps",
          example: "manage-apps",
          description: "Browse and edit registered applications, flags, and project-aware launch arguments.",
          tag: "Apps",
          tagColor: "bg-blue-950/60 border-blue-800/80 text-blue-300",
        },
        {
          syntax: "add-group",
          example: "add-group",
          description: "Create an application group and customize its launch sequence.",
          tag: "Groups",
          tagColor: "bg-purple-950/60 border-purple-800/80 text-purple-300",
        },
        {
          syntax: "manage-groups",
          example: "manage-groups",
          description: "Manage groups and assign which group is activated by the '//' workspace shortcut.",
          tag: "Groups",
          tagColor: "bg-purple-950/60 border-purple-800/80 text-purple-300",
        },
        {
          syntax: "help",
          example: "help",
          description: "Opens this interactive quickstart and usage guide.",
          tag: "Help",
          tagColor: "bg-neutral-800 border-neutral-700 text-neutral-300",
        },
        {
          syntax: "settings",
          example: "settings",
          description: "Configure system startup autostart, global summon hotkey, and default group.",
          tag: "Settings",
          tagColor: "bg-emerald-950/60 border-emerald-800/80 text-emerald-300",
        },
      ],
    },
    {
      id: "variables",
      category: "variables",
      title: "App Argument Interpolation Variables",
      items: [
        {
          syntax: "{PROJECT_PATH}",
          example: 'e.g. Code.exe "{PROJECT_PATH}"',
          description: "Replaced with the project's absolute folder path on disk.",
          tag: "Variable",
          tagColor: "bg-cyan-950/60 border-cyan-800/80 text-cyan-300",
        },
        {
          syntax: "{PROJECT_URL}",
          example: 'e.g. chrome.exe "{PROJECT_URL}"',
          description: "Replaced with the project's web URL (e.g. http://localhost:3000).",
          tag: "Variable",
          tagColor: "bg-cyan-950/60 border-cyan-800/80 text-cyan-300",
        },
        {
          syntax: "{PROJECT_NAME}",
          example: 'e.g. wt.exe --title "{PROJECT_NAME}"',
          description: "Replaced with the human-readable project title.",
          tag: "Variable",
          tagColor: "bg-cyan-950/60 border-cyan-800/80 text-cyan-300",
        },
        {
          syntax: "{PROJECT_COMMAND}",
          example: "e.g. {PROJECT_COMMAND}",
          description: "Replaced with the project's shortcut command slug.",
          tag: "Variable",
          tagColor: "bg-cyan-950/60 border-cyan-800/80 text-cyan-300",
        },
      ],
    },
    {
      id: "shortcuts",
      category: "shortcuts",
      title: "Keyboard Shortcuts",
      items: [
        {
          syntax: "Alt + Space",
          example: "Alt + Space",
          description: "Toggle launcher window from anywhere in Windows (customizable).",
          tag: "Global",
          tagColor: "bg-rose-950/60 border-rose-800/80 text-rose-300",
        },
        {
          syntax: "Tab",
          example: "Tab",
          description: "Autocomplete selected suggestion in search bar.",
          tag: "Navigation",
          tagColor: "bg-neutral-800 border-neutral-700 text-neutral-300",
        },
        {
          syntax: "↑ / ↓ Arrows",
          example: "Up / Down",
          description: "Navigate up and down through matching suggestions.",
          tag: "Navigation",
          tagColor: "bg-neutral-800 border-neutral-700 text-neutral-300",
        },
        {
          syntax: "Ctrl + Enter",
          example: "Ctrl + Enter",
          description: "Save form changes in Add/Edit Project, App, or Group views.",
          tag: "Form",
          tagColor: "bg-neutral-800 border-neutral-700 text-neutral-300",
        },
        {
          syntax: "Esc",
          example: "Esc",
          description: "Close launcher or return to search bar from any sub-view.",
          tag: "Navigation",
          tagColor: "bg-neutral-800 border-neutral-700 text-neutral-300",
        },
      ],
    },
  ];

  const term = searchFilter.toLowerCase().trim();

  const filteredSections = sections
    .filter((sec) => activeTab === "all" || sec.category === activeTab)
    .map((sec) => ({
      ...sec,
      items: sec.items.filter(
        (item) =>
          !term ||
          item.syntax.toLowerCase().includes(term) ||
          item.example.toLowerCase().includes(term) ||
          item.description.toLowerCase().includes(term) ||
          item.tag.toLowerCase().includes(term)
      ),
    }))
    .filter((sec) => sec.items.length > 0);

  return (
    <div
      onKeyDown={handleKeyDown}
      className="flex flex-col max-h-[420px] overflow-hidden text-neutral-100"
    >
      {/* Header */}
      <div className="flex items-center justify-between px-5 py-3 border-b border-neutral-800/80 bg-neutral-950/80">
        <div className="flex items-center space-x-2.5">
          <button
            type="button"
            onClick={onCancel}
            title="Return to launcher (Esc)"
            className="p-1 rounded-md text-neutral-400 hover:text-neutral-200 hover:bg-neutral-800/60 transition-colors cursor-pointer"
          >
            ←
          </button>
          <div className="flex items-center space-x-2">
            <HelpIcon className="w-4 h-4 text-blue-400" />
            <h2 className="text-sm font-semibold text-neutral-100">
              Universal Launcher &bull; Help & Quickstart
            </h2>
          </div>
        </div>

        <button
          type="button"
          onClick={onCancel}
          title="Close (Esc)"
          className="p-1 rounded-md text-neutral-400 hover:text-neutral-200 hover:bg-neutral-800/60 transition-colors cursor-pointer"
        >
          <CloseIcon className="w-4 h-4" />
        </button>
      </div>

      {/* Filter and Category Tabs */}
      <div className="px-5 py-2.5 border-b border-neutral-800/60 bg-neutral-900/40 flex items-center justify-between space-x-3">
        <div className="flex items-center space-x-2 flex-1 min-w-0">
          <SearchIcon className="w-4 h-4 text-neutral-500 shrink-0" />
          <input
            autoFocus
            type="text"
            placeholder="Search commands, syntax, shortcuts..."
            value={searchFilter}
            onChange={(e) => setSearchFilter(e.target.value)}
            className="w-full bg-transparent text-xs text-neutral-200 placeholder-neutral-500 focus:outline-none"
          />
        </div>

        {/* Tab Filters */}
        <div className="flex items-center space-x-1 shrink-0">
          {[
            { id: "all", label: "All" },
            { id: "syntax", label: "Syntax" },
            { id: "management", label: "Commands" },
            { id: "variables", label: "Variables" },
            { id: "shortcuts", label: "Shortcuts" },
          ].map((tab) => (
            <button
              key={tab.id}
              type="button"
              onClick={() => setActiveTab(tab.id)}
              className={`px-2 py-0.5 rounded text-[11px] font-medium transition-colors cursor-pointer ${
                activeTab === tab.id
                  ? "bg-blue-600 text-white shadow-sm"
                  : "text-neutral-400 hover:text-neutral-200 hover:bg-neutral-800/50"
              }`}
            >
              {tab.label}
            </button>
          ))}
        </div>
      </div>

      {/* Guide Content */}
      <div className="p-4 space-y-4 overflow-y-auto max-h-[295px]">
        {filteredSections.length === 0 ? (
          <div className="text-center py-8 text-xs text-neutral-500">
            No help topics match "{searchFilter}".
          </div>
        ) : (
          filteredSections.map((sec) => (
            <div key={sec.id} className="space-y-2">
              <h3 className="text-[11px] font-bold uppercase tracking-wider text-neutral-400 px-1">
                {sec.title}
              </h3>

              <div className="space-y-1.5">
                {sec.items.map((item, idx) => (
                  <div
                    key={idx}
                    className="flex items-start justify-between p-2.5 rounded-xl bg-neutral-900/50 border border-neutral-800/70 hover:border-neutral-700/80 transition-colors group"
                  >
                    <div className="space-y-1 min-w-0 pr-3">
                      <div className="flex items-center space-x-2">
                        <code className="px-2 py-0.5 rounded bg-neutral-800 border border-neutral-700 font-mono text-xs text-blue-300 font-medium">
                          {item.syntax}
                        </code>
                        <span
                          className={`text-[10px] px-1.5 py-0.2 rounded border font-medium ${item.tagColor}`}
                        >
                          {item.tag}
                        </span>
                      </div>
                      <p className="text-xs text-neutral-300 leading-relaxed">
                        {item.description}
                      </p>
                    </div>

                    <div className="text-right shrink-0 pt-0.5">
                      <span className="text-[10px] text-neutral-500 font-mono block">
                        Example:
                      </span>
                      <code className="text-[11px] font-mono text-neutral-400 bg-neutral-950/60 px-1.5 py-0.5 rounded border border-neutral-800 block mt-0.5">
                        {item.example}
                      </code>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          ))
        )}
      </div>

      {/* Footer */}
      <div className="flex items-center justify-between px-5 py-2 border-t border-neutral-800/80 bg-neutral-950/90 text-[11px] text-neutral-500">
        <span>Press <kbd className="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 text-neutral-400 font-mono text-[10px]">Alt + Space</kbd> to toggle launcher</span>
        <span>esc to return</span>
      </div>
    </div>
  );
}
