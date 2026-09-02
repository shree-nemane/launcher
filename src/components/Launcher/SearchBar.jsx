import React, { useEffect } from "react";
import { SearchIcon, CloseIcon, HelpIcon } from "./Icons";

export function SearchBar({
  input,
  inputRef,
  isExecuting,
  hasSuggestions,
  onChange,
  onKeyDown,
  onClear,
  onOpenHelp,
  onClose,
}) {
  // Ensure input gets focused as soon as the search bar is rendered
  useEffect(() => {
    inputRef.current?.focus();
  }, [inputRef]);

  return (
    <div
      data-tauri-drag-region
      className="relative flex items-center h-14 px-4 bg-neutral-950/90 border-b border-neutral-800/80 cursor-default"
    >
      {/* 2px Animated Indeterminate Progress Line while executing */}
      {isExecuting && (
        <div
          role="progressbar"
          aria-label="Executing command"
          className="absolute top-0 left-0 right-0 h-[2px] overflow-hidden bg-neutral-800"
        >
          <div className="w-full h-full bg-gradient-to-r from-blue-500 via-indigo-500 to-emerald-400 animate-pulse" />
        </div>
      )}

      {/* Search Icon */}
      <div className="text-neutral-400 mr-3 shrink-0" aria-hidden="true">
        <SearchIcon className="w-5 h-5 text-neutral-400" />
      </div>

      {/* Main Search Input */}
      <input
        ref={inputRef}
        autoFocus
        type="text"
        role="combobox"
        aria-autocomplete="list"
        aria-expanded={hasSuggestions}
        aria-controls="suggestions-list"
        aria-label="Command search"
        value={input}
        onChange={onChange}
        onKeyDown={onKeyDown}
        placeholder="Type a project or command (/v, //, deepfake)..."
        disabled={isExecuting}
        spellCheck={false}
        autoComplete="off"
        className="w-full bg-transparent text-[16px] text-neutral-100 placeholder-neutral-500 focus:outline-none font-normal"
      />

      {/* Right Controls Container */}
      <div className="flex items-center space-x-2 shrink-0 ml-2">
        {/* Distinct Clear Search Text Button (Only appears when user has typed text) */}
        {input && (
          <button
            type="button"
            onClick={onClear}
            aria-label="Clear search input"
            title="Clear search text"
            className="p-1 rounded-full text-neutral-400 hover:text-neutral-100 hover:bg-neutral-800 transition-colors cursor-pointer"
          >
            <CloseIcon className="w-3.5 h-3.5" />
          </button>
        )}

        {/* Subtle Divider */}
        <div className="w-[1px] h-4 bg-neutral-800/90" />

        {/* Help Button (Takes minimal space) */}
        <button
          type="button"
          onClick={onOpenHelp}
          aria-label="Help and quickstart"
          title="Help & Quickstart Guide (help)"
          className="flex items-center space-x-1 px-2 py-1 rounded-md text-neutral-400 hover:text-blue-300 hover:bg-neutral-800/70 transition-colors cursor-pointer text-xs font-medium"
        >
          <HelpIcon className="w-3.5 h-3.5" />
          <span>Help</span>
        </button>

        {/* Window Close Button */}
        <button
          type="button"
          onClick={onClose}
          aria-label="Close launcher"
          title="Close Launcher (Esc)"
          className="p-1.5 rounded-md text-neutral-400 hover:text-rose-300 hover:bg-rose-950/40 transition-colors cursor-pointer"
        >
          <CloseIcon className="w-3.5 h-3.5" />
        </button>
      </div>
    </div>
  );
}
