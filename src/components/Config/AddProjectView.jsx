import React, { useEffect, useRef } from "react";
import { FormField } from "./FormField";
import { RunCommandsEditor } from "./RunCommandsEditor";
import { useProjectForm } from "../../hooks/useProjectForm";
import { FolderIcon, AlertIcon } from "../Launcher/Icons";

export function AddProjectView({ onCancel, onSuccess }) {
  const nameInputRef = useRef(null);
  const {
    formData,
    errors,
    isSaving,
    handleChange,
    handleBrowseFolder,
    handleSave,
  } = useProjectForm(onSuccess);

  useEffect(() => {
    nameInputRef.current?.focus();
  }, []);

  const handleKeyDown = (e) => {
    if (e.key === "Escape") {
      e.preventDefault();
      onCancel();
    } else if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
      e.preventDefault();
      handleSave();
    }
  };

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
          <div>
            <h2 className="text-sm font-semibold text-neutral-100">Add Project</h2>
          </div>
        </div>
        <span className="text-[11px] text-neutral-500 font-mono">
          Ctrl + ↵ to save
        </span>
      </div>

      {/* Form Content */}
      <div className="p-5 space-y-4 overflow-y-auto max-h-[310px]">
        {errors.general && (
          <div
            role="alert"
            className="flex items-center space-x-2 px-3.5 py-2.5 rounded-lg bg-rose-950/40 border border-rose-900/60 text-rose-300 text-xs font-medium"
          >
            <AlertIcon className="w-4 h-4 text-rose-400 shrink-0" />
            <span className="truncate">{errors.general}</span>
          </div>
        )}

        <div className="grid grid-cols-2 gap-3">
          <FormField label="Project Name" required error={errors.name}>
            <input
              ref={nameInputRef}
              type="text"
              placeholder="e.g. Deepfake Forensic AI"
              value={formData.name}
              onChange={(e) => handleChange("name", e.target.value)}
              className="w-full px-3 py-2 rounded-lg bg-neutral-900 border border-neutral-800 text-xs text-neutral-100 placeholder-neutral-500 focus:outline-none focus:border-blue-500/70"
            />
          </FormField>

          <FormField label="Command" required error={errors.command}>
            <input
              type="text"
              placeholder="e.g. deepfake"
              value={formData.command}
              onChange={(e) => handleChange("command", e.target.value)}
              className="w-full px-3 py-2 rounded-lg bg-neutral-900 border border-neutral-800 text-xs text-neutral-100 placeholder-neutral-500 focus:outline-none focus:border-blue-500/70 font-mono"
            />
          </FormField>
        </div>

        <FormField label="Project Directory" required error={errors.path}>
          <div className="flex items-center space-x-2">
            <input
              type="text"
              placeholder="D:\Projects\Deepfake"
              value={formData.path}
              onChange={(e) => handleChange("path", e.target.value)}
              className="flex-1 px-3 py-2 rounded-lg bg-neutral-900 border border-neutral-800 text-xs text-neutral-100 placeholder-neutral-500 focus:outline-none focus:border-blue-500/70 font-mono truncate"
            />
            <button
              type="button"
              onClick={handleBrowseFolder}
              className="flex items-center space-x-1.5 px-3 py-2 rounded-lg bg-neutral-800 hover:bg-neutral-700 text-xs font-medium text-neutral-200 transition-colors cursor-pointer shrink-0 border border-neutral-700"
            >
              <FolderIcon className="w-3.5 h-3.5 text-neutral-400" />
              <span>Browse</span>
            </button>
          </div>
        </FormField>

        <div className="grid grid-cols-2 gap-3">
          <FormField label="Project URL" helper="Optional" error={errors.url}>
            <input
              type="text"
              placeholder="http://localhost:5173"
              value={formData.url}
              onChange={(e) => handleChange("url", e.target.value)}
              className="w-full px-3 py-2 rounded-lg bg-neutral-900 border border-neutral-800 text-xs text-neutral-100 placeholder-neutral-500 focus:outline-none focus:border-blue-500/70 font-mono"
            />
          </FormField>

          <FormField
            label="Working Directory"
            helper="Optional"
            error={errors.workingDirectory}
          >
            <input
              type="text"
              placeholder="Defaults to project folder"
              value={formData.workingDirectory}
              onChange={(e) => handleChange("workingDirectory", e.target.value)}
              className="w-full px-3 py-2 rounded-lg bg-neutral-900 border border-neutral-800 text-xs text-neutral-100 placeholder-neutral-500 focus:outline-none focus:border-blue-500/70 font-mono"
            />
          </FormField>
        </div>

        <RunCommandsEditor
          commands={formData.runCommands}
          onChange={(cmds) => handleChange("runCommands", cmds)}
        />
      </div>

      {/* Footer Actions */}
      <div className="flex items-center justify-between px-5 py-3 border-t border-neutral-800/80 bg-neutral-950/90">
        <button
          type="button"
          onClick={onCancel}
          className="px-3.5 py-1.5 rounded-lg text-xs font-medium text-neutral-400 hover:text-neutral-200 hover:bg-neutral-800/60 transition-colors cursor-pointer"
        >
          Cancel
        </button>

        <button
          type="button"
          onClick={handleSave}
          disabled={isSaving}
          className="px-4 py-1.5 rounded-lg text-xs font-semibold bg-blue-600 hover:bg-blue-500 disabled:bg-blue-800/60 text-white shadow-md shadow-blue-900/30 transition-colors cursor-pointer flex items-center space-x-1.5"
        >
          {isSaving ? (
            <span>Saving...</span>
          ) : (
            <span>Save Project</span>
          )}
        </button>
      </div>
    </div>
  );
}
