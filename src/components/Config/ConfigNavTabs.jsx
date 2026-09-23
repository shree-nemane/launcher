import React from "react";
import { FolderIcon, AppIcon, GroupIcon, SystemIcon } from "../Launcher/Icons";

export function ConfigNavTabs({ activeTab, onNavigate }) {
  if (!onNavigate) return null;

  const tabs = [
    { id: "manageProjects", label: "Projects", icon: FolderIcon },
    { id: "manageApps", label: "Applications", icon: AppIcon },
    { id: "manageGroups", label: "Groups", icon: GroupIcon },
    { id: "settings", label: "Settings", icon: SystemIcon },
  ];

  return (
    <div className="flex items-center space-x-1 px-5 py-2 bg-neutral-900/40 border-b border-neutral-800/60">
      {tabs.map((tab) => {
        const Icon = tab.icon;
        const isActive = activeTab === tab.id;
        return (
          <button
            key={tab.id}
            type="button"
            onClick={() => onNavigate(tab.id)}
            className={`flex items-center space-x-1.5 px-3 py-1 rounded-md text-xs font-medium transition-colors cursor-pointer ${
              isActive
                ? "bg-neutral-800 text-blue-400 font-semibold shadow-xs"
                : "text-neutral-400 hover:text-neutral-200 hover:bg-neutral-800/50"
            }`}
          >
            <Icon className="w-3.5 h-3.5" />
            <span>{tab.label}</span>
          </button>
        );
      })}
    </div>
  );
}
