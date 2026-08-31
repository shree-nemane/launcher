import React, { useEffect, useRef } from "react";
import { FormField } from "./FormField";
import { ArgumentsEditor } from "./ArgumentsEditor";
import { VariableChips } from "./VariableChips";
import { useApplicationForm } from "../../hooks/useApplicationForm";
import { AppIcon, AlertIcon } from "../Launcher/Icons";

export function AddAppView({ onCancel, onSuccess }) {
  const nameInputRef = useRef(null);
  const projectArgsRef = useRef(null);

  const {
    formData,
    errors,
    isSaving,
    handleChange,
    handleBrowseExecutable,
    handleSave,
  } = useApplicationForm(onSuccess);

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

  const handleInsertVariable = (varText) => {
    projectArgsRef.current?.insertVariable(varText);
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
            <h2 className="text-sm font-semibold text-neutral-100">Add Application</h2>
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
          <FormField label="Application Name" required error={errors.name}>
            <input
              ref={nameInputRef}
              type="text"
              placeholder="e.g. VS Code"
              value={formData.name}
              onChange={(e) => handleChange("name", e.target.value)}
              className="w-full px-3 py-2 rounded-lg bg-neutral-900 border border-neutral-800 text-xs text-neutral-100 placeholder-neutral-500 focus:outline-none focus:border-blue-500/70"
            />
          </FormField>

          <FormField
            label="Command"
            required
            helper="Must start with /"
            error={errors.command}
          >
            <input
              type="text"
              placeholder="e.g. /v"
              value={formData.command}
              onChange={(e) => handleChange("command", e.target.value)}
              className="w-full px-3 py-2 rounded-lg bg-neutral-900 border border-neutral-800 text-xs text-neutral-100 placeholder-neutral-500 focus:outline-none focus:border-blue-500/70 font-mono"
            />
          </FormField>
        </div>

        <FormField label="Executable Path" required error={errors.executablePath}>
          <div className="flex items-center space-x-2">
            <input
              type="text"
              placeholder="C:\Program Files\Microsoft VS Code\Code.exe"
              value={formData.executablePath}
              onChange={(e) => handleChange("executablePath", e.target.value)}
              className="flex-1 px-3 py-2 rounded-lg bg-neutral-900 border border-neutral-800 text-xs text-neutral-100 placeholder-neutral-500 focus:outline-none focus:border-blue-500/70 font-mono truncate"
            />
            <button
              type="button"
              onClick={handleBrowseExecutable}
              className="flex items-center space-x-1.5 px-3 py-2 rounded-lg bg-neutral-800 hover:bg-neutral-700 text-xs font-medium text-neutral-200 transition-colors cursor-pointer shrink-0 border border-neutral-700"
            >
              <AppIcon className="w-3.5 h-3.5 text-neutral-400" />
              <span>Browse</span>
            </button>
          </div>
        </FormField>

        {/* Normal Launch Arguments */}
        <ArgumentsEditor
          title="Normal Launch Arguments"
          helper="Optional"
          argumentsList={formData.normalArguments}
          onChange={(args) => handleChange("normalArguments", args)}
          placeholder="e.g. --new-window"
        />

        {/* Project Launch Section */}
        <div className="p-3 rounded-xl bg-neutral-900/60 border border-neutral-800/80 space-y-3">
          <div className="flex items-center justify-between">
            <div>
              <span className="text-xs font-medium text-neutral-200 block">
                Project-Aware Launch
              </span>
              <span className="text-[11px] text-neutral-500">
                Pass project path or URL when launched with a project
              </span>
            </div>
            <label className="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                checked={formData.projectLaunchEnabled}
                onChange={(e) =>
                  handleChange("projectLaunchEnabled", e.target.checked)
                }
                className="sr-only peer"
              />
              <div className="w-9 h-5 bg-neutral-800 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-neutral-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-blue-600"></div>
            </label>
          </div>

          {formData.projectLaunchEnabled && (
            <div className="space-y-3 pt-2 border-t border-neutral-800/60">
              <ArgumentsEditor
                ref={projectArgsRef}
                title="Project Launch Arguments"
                argumentsList={formData.projectArguments}
                onChange={(args) => handleChange("projectArguments", args)}
                placeholder="e.g. {PROJECT_PATH}"
              />

              <VariableChips onInsert={handleInsertVariable} />
            </div>
          )}
        </div>

        {/* Working Directory */}
        <FormField
          label="Working Directory"
          helper="Optional"
          error={errors.workingDirectory}
        >
          <input
            type="text"
            placeholder="Defaults to project folder or app path"
            value={formData.workingDirectory}
            onChange={(e) => handleChange("workingDirectory", e.target.value)}
            className="w-full px-3 py-2 rounded-lg bg-neutral-900 border border-neutral-800 text-xs text-neutral-100 placeholder-neutral-500 focus:outline-none focus:border-blue-500/70 font-mono"
          />
        </FormField>
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
          {isSaving ? <span>Saving...</span> : <span>Save Application</span>}
        </button>
      </div>
    </div>
  );
}
