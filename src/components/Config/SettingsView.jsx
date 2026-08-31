import React, { useState, useEffect } from "react";
import { launcherService } from "../../services/launcherService";
import { SystemIcon, CloseIcon, AlertIcon } from "../Launcher/Icons";

export function SettingsView({ onCancel, onSaved }) {
  const [autostartEnabled, setAutostartEnabled] = useState(false);
  const [globalShortcut, setGlobalShortcut] = useState("Alt+Space");
  const [defaultGroupId, setDefaultGroupId] = useState("");
  const [groups, setGroups] = useState([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);
  const [notice, setNotice] = useState(null);
  const [error, setError] = useState(null);

  useEffect(() => {
    async function loadData() {
      try {
        const [settings, groupsList, isAuto] = await Promise.all([
          launcherService.getSettings(),
          launcherService.getGroups(),
          launcherService.isAutostartEnabled(),
        ]);

        if (settings) {
          setGlobalShortcut(settings.globalShortcut || "Alt+Space");
          setDefaultGroupId(settings.defaultApplicationGroupId || "");
        }
        setGroups(groupsList || []);
        setAutostartEnabled(Boolean(isAuto));
      } catch (err) {
        console.error("Failed to load settings:", err);
        setError("Failed to load settings from storage");
      } finally {
        setIsLoading(false);
      }
    }

    loadData();
  }, []);

  const handleToggleAutostart = async () => {
    setError(null);
    setNotice(null);
    try {
      if (autostartEnabled) {
        await launcherService.disableAutostart();
        setAutostartEnabled(false);
        setNotice("Launch on system startup disabled.");
      } else {
        await launcherService.enableAutostart();
        setAutostartEnabled(true);
        setNotice("Launch on system startup enabled.");
      }
    } catch (err) {
      console.error("Failed to toggle autostart:", err);
      setError("Failed to update system startup setting.");
    }
  };

  const handleSaveSettings = async (e) => {
    if (e) e.preventDefault();
    setIsSaving(true);
    setError(null);
    setNotice(null);

    try {
      const current = await launcherService.getSettings();
      const updated = await launcherService.updateSettings({
        ...current,
        globalShortcut: globalShortcut.trim() || "Alt+Space",
        defaultApplicationGroupId: defaultGroupId || null,
      });

      setNotice("Settings saved successfully.");
      if (onSaved) {
        onSaved(updated);
      }
    } catch (err) {
      console.error("Failed to save settings:", err);
      setError(typeof err === "string" ? err : err.message || "Failed to save settings");
    } finally {
      setIsSaving(false);
    }
  };

  const handleKeyDown = (e) => {
    if (e.key === "Escape") {
      e.preventDefault();
      onCancel();
    } else if (e.key === "Enter" && e.ctrlKey) {
      e.preventDefault();
      handleSaveSettings();
    }
  };

  const shortcutPresets = ["Alt+Space", "Ctrl+Space", "Alt+Shift+Space", "Ctrl+Alt+Space"];

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
            <SystemIcon className="w-4 h-4 text-blue-400" />
            <h2 className="text-sm font-semibold text-neutral-100">
              Launcher Settings
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

      {/* Notice & Error */}
      {notice && (
        <div className="px-4 py-2 mx-4 mt-3 rounded-lg bg-emerald-950/50 border border-emerald-900/60 text-xs text-emerald-300">
          {notice}
        </div>
      )}
      {error && (
        <div className="px-4 py-2 mx-4 mt-3 rounded-lg bg-rose-950/50 border border-rose-900/60 text-xs text-rose-300 flex items-center space-x-2">
          <AlertIcon className="w-4 h-4 shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {/* Content Form */}
      <div className="p-4 space-y-4 overflow-y-auto max-h-[300px]">
        {/* Startup Setting */}
        <div className="p-3.5 rounded-xl bg-neutral-900/40 border border-neutral-800/70 flex items-center justify-between">
          <div className="space-y-0.5 pr-4">
            <div className="text-xs font-semibold text-neutral-200 flex items-center space-x-2">
              <span>Launch on system startup</span>
              <span
                className={`text-[10px] px-1.5 py-0.5 rounded font-mono ${
                  autostartEnabled
                    ? "bg-emerald-950/80 border border-emerald-800 text-emerald-300"
                    : "bg-neutral-800 border border-neutral-700 text-neutral-400"
                }`}
              >
                {autostartEnabled ? "ON" : "OFF"}
              </span>
            </div>
            <p className="text-[11px] text-neutral-400 leading-relaxed">
              Start Universal Launcher hidden in the system tray automatically when you log into Windows.
            </p>
          </div>

          <button
            type="button"
            onClick={handleToggleAutostart}
            disabled={isLoading}
            className={`relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none ${
              autostartEnabled ? "bg-blue-600" : "bg-neutral-700"
            }`}
          >
            <span
              className={`pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out ${
                autostartEnabled ? "translate-x-5" : "translate-x-0"
              }`}
            />
          </button>
        </div>

        {/* Global Shortcut Setting */}
        <div className="p-3.5 rounded-xl bg-neutral-900/40 border border-neutral-800/70 space-y-2.5">
          <div className="flex items-center justify-between">
            <label className="text-xs font-semibold text-neutral-200 block">
              Global Summon Shortcut
            </label>
            <span className="text-[10px] text-neutral-500 font-mono">
              Summon launcher anywhere
            </span>
          </div>

          <div className="flex items-center space-x-2">
            <input
              type="text"
              value={globalShortcut}
              onChange={(e) => setGlobalShortcut(e.target.value)}
              placeholder="e.g. Alt+Space"
              className="flex-1 bg-neutral-950/90 border border-neutral-700/80 rounded-lg px-3 py-1.5 text-xs text-blue-300 font-mono focus:outline-none focus:border-blue-500 transition-colors"
            />
          </div>

          {/* Shortcut presets */}
          <div className="flex items-center space-x-1.5 pt-1">
            <span className="text-[10px] text-neutral-500 mr-1">Presets:</span>
            {shortcutPresets.map((preset) => (
              <button
                key={preset}
                type="button"
                onClick={() => setGlobalShortcut(preset)}
                className={`px-2 py-0.5 rounded text-[10px] font-mono border transition-colors cursor-pointer ${
                  globalShortcut === preset
                    ? "bg-blue-950/70 border-blue-700 text-blue-300"
                    : "bg-neutral-800/80 border-neutral-700 text-neutral-400 hover:text-neutral-200 hover:bg-neutral-800"
                }`}
              >
                {preset}
              </button>
            ))}
          </div>
        </div>

        {/* Default Application Group */}
        <div className="p-3.5 rounded-xl bg-neutral-900/40 border border-neutral-800/70 space-y-2">
          <div className="flex items-center justify-between">
            <label className="text-xs font-semibold text-neutral-200 block">
              Default Workspace Group (//)
            </label>
            <span className="text-[10px] text-neutral-500">
              Triggered by // shortcut
            </span>
          </div>

          <select
            value={defaultGroupId}
            onChange={(e) => setDefaultGroupId(e.target.value)}
            className="w-full bg-neutral-950/90 border border-neutral-700/80 rounded-lg px-3 py-1.5 text-xs text-neutral-200 focus:outline-none focus:border-blue-500 transition-colors cursor-pointer"
          >
            <option value="">-- No default group selected --</option>
            {groups.map((grp) => (
              <option key={grp.id} value={grp.id}>
                {grp.name} ({grp.applicationIds.length} app{grp.applicationIds.length === 1 ? "" : "s"})
              </option>
            ))}
          </select>
        </div>
      </div>

      {/* Footer */}
      <div className="flex items-center justify-between px-5 py-2.5 border-t border-neutral-800/80 bg-neutral-950/90 text-xs">
        <span className="text-[11px] text-neutral-500">
          Press <kbd className="px-1 py-0.5 rounded bg-neutral-800 text-neutral-400 font-mono text-[10px]">Ctrl+Enter</kbd> to save
        </span>

        <div className="flex items-center space-x-2">
          <button
            type="button"
            onClick={onCancel}
            className="px-3 py-1 rounded-lg text-xs font-medium text-neutral-400 hover:text-neutral-200 hover:bg-neutral-800/60 transition-colors cursor-pointer"
          >
            Cancel
          </button>
          <button
            type="button"
            onClick={handleSaveSettings}
            disabled={isSaving}
            className="px-3.5 py-1 rounded-lg text-xs font-semibold bg-blue-600 hover:bg-blue-500 text-white transition-colors cursor-pointer shadow-sm shadow-blue-900/30"
          >
            {isSaving ? "Saving..." : "Save Settings"}
          </button>
        </div>
      </div>
    </div>
  );
}