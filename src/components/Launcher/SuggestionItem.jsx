import React from "react";
import { FolderIcon, AppIcon, GroupIcon, SystemIcon } from "./Icons";

export function SuggestionItem({
  suggestion,
  isSelected,
  onSelect,
  onMouseEnter,
}) {
  const getIcon = () => {
    switch (suggestion.kind) {
      case "project":
        return <FolderIcon className="w-4 h-4 text-amber-400/90" />;
      case "app":
        return <AppIcon className="w-4 h-4 text-blue-400/90" />;
      case "group":
        return <GroupIcon className="w-4 h-4 text-emerald-400/90" />;
      case "system":
        return <SystemIcon className="w-4 h-4 text-purple-400/90" />;
      default:
        return <AppIcon className="w-4 h-4 text-neutral-400" />;
    }
  };

  return (
    <div
      role="option"
      aria-selected={isSelected}
      onClick={() => onSelect(suggestion)}
      onMouseEnter={onMouseEnter}
      className={`flex items-center justify-between px-4 py-2.5 rounded-lg cursor-pointer transition-colors text-sm ${
        isSelected
          ? "bg-neutral-800/90 text-neutral-100"
          : "text-neutral-300 hover:bg-neutral-900/60"
      }`}
    >
      <div className="flex items-center space-x-3 min-w-0 pr-2">
        <div className="shrink-0" aria-hidden="true">
          {getIcon()}
        </div>
        <span className="font-mono text-xs px-2 py-0.5 rounded bg-neutral-800/90 border border-neutral-700 text-neutral-200 shrink-0">
          {suggestion.command}
        </span>
        <span className="font-medium truncate text-neutral-100">
          {suggestion.name}
        </span>
      </div>

      {suggestion.description && (
        <span className="text-xs text-neutral-500 truncate max-w-[200px] shrink-0">
          {suggestion.description}
        </span>
      )}
    </div>
  );
}
