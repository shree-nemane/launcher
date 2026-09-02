import React from "react";

export function ShortcutFooter({ onOpenHelp }) {
  return (
    <div
      data-tauri-drag-region
      className="flex items-center justify-between px-4 py-2 border-t border-neutral-800/80 bg-neutral-950/40 text-[11px] text-neutral-500 cursor-default"
    >
      <div className="flex items-center space-x-3">
        <span className="flex items-center space-x-1">
          <kbd className="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 text-neutral-400 font-mono text-[10px]">
            ↵
          </kbd>
          <span>Execute</span>
        </span>

        <span className="flex items-center space-x-1">
          <kbd className="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 text-neutral-400 font-mono text-[10px]">
            ⇥
          </kbd>
          <span>Complete</span>
        </span>

        <span className="flex items-center space-x-1">
          <kbd className="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 text-neutral-400 font-mono text-[10px]">
            ↑↓
          </kbd>
          <span>Navigate</span>
        </span>
      </div>

      <div className="flex items-center space-x-2.5">
        <button
          type="button"
          onClick={onOpenHelp}
          className="flex items-center space-x-1 text-neutral-400 hover:text-blue-400 transition-colors cursor-pointer"
        >
          <kbd className="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 text-neutral-400 font-mono text-[10px]">
            help
          </kbd>
          <span>Guide</span>
        </button>

        <span className="flex items-center space-x-1">
          <kbd className="px-1.5 py-0.5 rounded bg-neutral-800 border border-neutral-700 text-neutral-400 font-mono text-[10px]">
            esc
          </kbd>
          <span>Close</span>
        </span>
      </div>
    </div>
  );
}
