import React, { useState, useEffect, useRef } from "react";
import { launcherService } from "../../services/launcherService";
import { FormField } from "./FormField";
import { AppIcon, AlertIcon, CloseIcon } from "../Launcher/Icons";

export function GroupFormView({ initialData = null, onCancel, onSuccess }) {
  const nameInputRef = useRef(null);
  const mode = initialData?.id ? "edit" : "create";

  const [name, setName] = useState(initialData?.name || "");
  const [selectedAppIds, setSelectedAppIds] = useState(
    initialData?.executionOrder?.length > 0
      ? initialData.executionOrder
      : initialData?.applications || []
  );
  const [isDefault, setIsDefault] = useState(false);
  const [availableApps, setAvailableApps] = useState([]);
  const [isSaving, setIsSaving] = useState(false);
  const [errors, setErrors] = useState({});

  useEffect(() => {
    nameInputRef.current?.focus();

    async function loadData() {
      try {
        const [appsResult, settingsResult] = await Promise.allSettled([
          launcherService.getApplications(),
          launcherService.getSettings(),
        ]);
        if (appsResult.status === "fulfilled") {
          setAvailableApps(appsResult.value || []);
        }
        if (
          settingsResult.status === "fulfilled" &&
          initialData?.id &&
          settingsResult.value?.defaultApplicationGroupId === initialData.id
        ) {
          setIsDefault(true);
        }
      } catch (err) {
        console.error("Failed to load apps/settings in GroupFormView:", err);
      }
    }
    loadData();
  }, [initialData]);

  const handleKeyDown = (e) => {
    if (e.key === "Escape") {
      e.preventDefault();
      onCancel();
    } else if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
      e.preventDefault();
      handleSave();
    }
  };

  const handleToggleApp = (appId) => {
    if (selectedAppIds.includes(appId)) {
      setSelectedAppIds(selectedAppIds.filter((id) => id !== appId));
    } else {
      setSelectedAppIds([...selectedAppIds, appId]);
    }
  };

  const handleMoveUp = (index) => {
    if (index <= 0) return;
    const next = [...selectedAppIds];
    const temp = next[index - 1];
    next[index - 1] = next[index];
    next[index] = temp;
    setSelectedAppIds(next);
  };

  const handleMoveDown = (index) => {
    if (index >= selectedAppIds.length - 1) return;
    const next = [...selectedAppIds];
    const temp = next[index + 1];
    next[index + 1] = next[index];
    next[index] = temp;
    setSelectedAppIds(next);
  };

  const handleSave = async () => {
    const trimmedName = name.trim();
    const newErrors = {};

    if (!trimmedName) {
      newErrors.name = "Group name is required";
    }
    if (selectedAppIds.length === 0) {
      newErrors.apps = "Select at least one application for this group";
    }

    if (Object.keys(newErrors).length > 0) {
      setErrors(newErrors);
      return;
    }

    setIsSaving(true);
    setErrors({});

    try {
      const payload = {
        id: initialData?.id || "",
        name: trimmedName,
        applications: selectedAppIds,
        executionOrder: selectedAppIds,
        createdAt: initialData?.createdAt || "",
        updatedAt: initialData?.updatedAt || "",
      };

      let savedGroup;
      if (mode === "edit") {
        savedGroup = await launcherService.updateGroup(payload);
      } else {
        savedGroup = await launcherService.createGroup(payload);
      }

      // Update default group setting if toggled
      const settings = await launcherService.getSettings();
      if (isDefault) {
        await launcherService.setDefaultGroup(savedGroup.id);
      } else if (settings?.defaultApplicationGroupId === savedGroup.id) {
        await launcherService.setDefaultGroup(null);
      }

      if (onSuccess) {
        onSuccess(savedGroup);
      }
    } catch (err) {
      let msg = "Failed to save application group";
      if (err && typeof err === "object") {
        msg = err.message || JSON.stringify(err);
      } else if (typeof err === "string") {
        msg = err;
      }
      setErrors({ general: msg });
    } finally {
      setIsSaving(false);
    }
  };

  const appMap = new Map(availableApps.map((a) => [a.id, a]));
  const title = mode === "edit" ? "Edit Application Group" : "Add Application Group";

  return (
    <div
      onKeyDown={handleKeyDown}
      className="flex flex-col max-h-[420px] overflow-hidden text-neutral-100"
    >
      {/* Header */}
      <div data-tauri-drag-region className="flex items-center justify-between px-5 py-3 border-b border-neutral-800/80 bg-neutral-950/80 cursor-default">
        <div className="flex items-center space-x-2.5">
          <button
            type="button"
            onClick={onCancel}
            title="Return (Esc)"
            className="p-1 rounded-md text-neutral-400 hover:text-neutral-200 hover:bg-neutral-800/60 transition-colors cursor-pointer"
          >
            ←
          </button>
          <div>
            <h2 className="text-sm font-semibold text-neutral-100">{title}</h2>
          </div>
        </div>
        <div className="flex items-center space-x-2.5">
          <span className="text-[11px] text-neutral-500 font-mono">
            Ctrl + ↵ to save
          </span>
          <button
            type="button"
            onClick={onCancel}
            title="Close / Cancel (Esc)"
            className="p-1 rounded-md text-neutral-400 hover:text-neutral-200 hover:bg-neutral-800/60 transition-colors cursor-pointer"
          >
            <CloseIcon className="w-4 h-4" />
          </button>
        </div>
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

        <div className="grid grid-cols-3 gap-3 items-start">
          <div className="col-span-2">
            <FormField label="Group Name" required error={errors.name}>
              <input
                ref={nameInputRef}
                type="text"
                placeholder="e.g. Full Stack Development"
                value={name}
                onChange={(e) => setName(e.target.value)}
                className="w-full px-3 py-2 rounded-lg bg-neutral-900 border border-neutral-800 text-xs text-neutral-100 placeholder-neutral-500 focus:outline-none focus:border-blue-500/70"
              />
            </FormField>
          </div>

          <div className="p-2.5 rounded-xl bg-neutral-900/60 border border-neutral-800/80 flex flex-col justify-between h-[66px]">
            <div className="flex items-center justify-between">
              <span className="text-xs font-medium text-neutral-200">
                Default for //
              </span>
              <label className="relative inline-flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  checked={isDefault}
                  onChange={(e) => setIsDefault(e.target.checked)}
                  className="sr-only peer"
                />
                <div className="w-8 h-4 bg-neutral-800 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-neutral-300 after:border after:rounded-full after:h-3 after:w-3 after:transition-all peer-checked:bg-blue-600"></div>
              </label>
            </div>
            <span className="text-[10px] text-neutral-500">
              Assign to <code className="text-blue-400">//</code> workspace shortcut
            </span>
          </div>
        </div>

        {/* Application Selection & Reordering */}
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <label className="text-xs font-medium text-neutral-300">
              Group Applications & Execution Order <span className="text-rose-400">*</span>
            </label>
            <span className="text-[11px] text-neutral-500">
              {selectedAppIds.length} selected
            </span>
          </div>

          {errors.apps && (
            <p className="text-[11px] text-rose-400">{errors.apps}</p>
          )}

          {/* Selected Execution Order List */}
          {selectedAppIds.length > 0 && (
            <div className="p-2 rounded-xl bg-neutral-900/80 border border-neutral-800 space-y-1.5">
              <span className="text-[10px] text-neutral-500 uppercase tracking-wider font-semibold px-2 block">
                Launch Sequence (Runs in this order):
              </span>
              {selectedAppIds.map((appId, index) => {
                const app = appMap.get(appId);
                return (
                  <div
                    key={appId}
                    className="flex items-center justify-between px-3 py-1.5 rounded-lg bg-neutral-950/70 border border-neutral-800/80 text-xs"
                  >
                    <div className="flex items-center space-x-2.5 min-w-0 pr-2">
                      <span className="w-4 h-4 rounded-full bg-blue-950 border border-blue-800 text-blue-300 text-[10px] flex items-center justify-center font-mono shrink-0">
                        {index + 1}
                      </span>
                      <AppIcon className="w-3.5 h-3.5 text-blue-400 shrink-0" />
                      <span className="font-mono text-neutral-300 font-medium">
                        {app?.command || appId}
                      </span>
                      <span className="text-neutral-400 truncate">
                        {app?.name || "Unknown Application"}
                      </span>
                    </div>

                    <div className="flex items-center space-x-1 shrink-0">
                      <button
                        type="button"
                        disabled={index === 0}
                        onClick={() => handleMoveUp(index)}
                        title="Move Up"
                        className="px-1.5 py-0.5 rounded bg-neutral-800 text-neutral-300 hover:text-white disabled:opacity-30 cursor-pointer"
                      >
                        ▲
                      </button>
                      <button
                        type="button"
                        disabled={index === selectedAppIds.length - 1}
                        onClick={() => handleMoveDown(index)}
                        title="Move Down"
                        className="px-1.5 py-0.5 rounded bg-neutral-800 text-neutral-300 hover:text-white disabled:opacity-30 cursor-pointer"
                      >
                        ▼
                      </button>
                      <button
                        type="button"
                        onClick={() => handleToggleApp(appId)}
                        title="Remove from group"
                        className="p-1 rounded text-neutral-500 hover:text-rose-400 transition-colors cursor-pointer"
                      >
                        <CloseIcon className="w-3.5 h-3.5" />
                      </button>
                    </div>
                  </div>
                );
              })}
            </div>
          )}

          {/* Available Apps to Add/Toggle */}
          <div className="space-y-1.5 pt-1">
            <span className="text-[10px] text-neutral-500 uppercase tracking-wider font-semibold block">
              Toggle Applications:
            </span>
            {availableApps.length === 0 ? (
              <div className="text-xs text-neutral-500 p-3 text-center">
                No applications registered yet. Please add applications first.
              </div>
            ) : (
              <div className="grid grid-cols-2 gap-2 max-h-[140px] overflow-y-auto pr-1">
                {availableApps.map((app) => {
                  const isSelected = selectedAppIds.includes(app.id);
                  return (
                    <button
                      key={app.id}
                      type="button"
                      onClick={() => handleToggleApp(app.id)}
                      className={`flex items-center justify-between p-2 rounded-lg border text-left transition-colors cursor-pointer ${
                        isSelected
                          ? "bg-blue-950/40 border-blue-700/60 text-neutral-100"
                          : "bg-neutral-900/40 border-neutral-800 text-neutral-400 hover:bg-neutral-900 hover:text-neutral-200"
                      }`}
                    >
                      <div className="flex items-center space-x-2 min-w-0 pr-1">
                        <AppIcon
                          className={`w-3.5 h-3.5 shrink-0 ${
                            isSelected ? "text-blue-400" : "text-neutral-500"
                          }`}
                        />
                        <div className="min-w-0">
                          <span className="text-xs font-medium block truncate">
                            {app.name}
                          </span>
                          <span className="text-[10px] font-mono text-neutral-500 block">
                            {app.command}
                          </span>
                        </div>
                      </div>
                      <span
                        className={`text-[10px] font-bold px-1.5 py-0.5 rounded ${
                          isSelected
                            ? "bg-blue-600 text-white"
                            : "bg-neutral-800 text-neutral-400"
                        }`}
                      >
                        {isSelected ? "✓" : "+"}
                      </span>
                    </button>
                  );
                })}
              </div>
            )}
          </div>
        </div>
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
            <span>{mode === "edit" ? "Save Changes" : "Save Group"}</span>
          )}
        </button>
      </div>
    </div>
  );
}
