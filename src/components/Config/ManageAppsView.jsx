import React, { useState, useEffect, useCallback } from "react";
import { launcherService } from "../../services/launcherService";
import { AppIcon, EditIcon, TrashIcon, SearchIcon, AlertIcon, CloseIcon } from "../Launcher/Icons";

export function ManageAppsView({ onCancel, onAddNew, onEdit, onDataChanged }) {
  const [applications, setApplications] = useState([]);
  const [filterText, setFilterText] = useState("");
  const [appToDelete, setAppToDelete] = useState(null);
  const [isDeleting, setIsDeleting] = useState(false);
  const [error, setError] = useState(null);

  const loadApplications = useCallback(async () => {
    try {
      const list = await launcherService.getApplications();
      setApplications(list || []);
    } catch (err) {
      console.error("Failed to load applications:", err);
      setError("Failed to load applications list");
    }
  }, []);

  useEffect(() => {
    loadApplications();
  }, [loadApplications]);

  const handleKeyDown = (e) => {
    if (e.key === "Escape") {
      e.preventDefault();
      if (appToDelete) {
        setAppToDelete(null);
      } else {
        onCancel();
      }
    }
  };

  const confirmDelete = async () => {
    if (!appToDelete || isDeleting) return;
    setIsDeleting(true);
    setError(null);
    try {
      await launcherService.deleteApplication(appToDelete.id);
      setAppToDelete(null);
      await loadApplications();
      if (onDataChanged) {
        onDataChanged();
      }
    } catch (err) {
      let msg = "Failed to delete application";
      if (err && typeof err === "object") {
        msg = err.message || JSON.stringify(err);
      } else if (typeof err === "string") {
        msg = err;
      }
      setError(msg);
    } finally {
      setIsDeleting(false);
    }
  };

  const filtered = applications.filter((a) => {
    const term = filterText.toLowerCase();
    return (
      a.name.toLowerCase().includes(term) ||
      a.command.toLowerCase().includes(term) ||
      a.executablePath.toLowerCase().includes(term)
    );
  });

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
            title="Return to launcher (Esc)"
            className="p-1 rounded-md text-neutral-400 hover:text-neutral-200 hover:bg-neutral-800/60 transition-colors cursor-pointer"
          >
            ←
          </button>
          <div>
            <h2 className="text-sm font-semibold text-neutral-100">
              Manage Applications
            </h2>
          </div>
        </div>

        <div className="flex items-center space-x-2">
          <button
            type="button"
            onClick={onAddNew}
            className="px-3 py-1 rounded-lg text-xs font-semibold bg-blue-600 hover:bg-blue-500 text-white transition-colors cursor-pointer shadow-sm shadow-blue-900/30"
          >
            + Add Application
          </button>
          <button
            type="button"
            onClick={onCancel}
            title="Close (Esc)"
            className="p-1 rounded-md text-neutral-400 hover:text-neutral-200 hover:bg-neutral-800/60 transition-colors cursor-pointer"
          >
            <CloseIcon className="w-4 h-4" />
          </button>
        </div>
      </div>

      {/* Search Filter */}
      <div className="px-5 py-2.5 border-b border-neutral-800/60 bg-neutral-900/40 flex items-center space-x-2">
        <SearchIcon className="w-4 h-4 text-neutral-500 shrink-0" />
        <input
          autoFocus
          type="text"
          placeholder="Filter applications by name or command..."
          value={filterText}
          onChange={(e) => setFilterText(e.target.value)}
          className="w-full bg-transparent text-xs text-neutral-200 placeholder-neutral-500 focus:outline-none"
        />
      </div>

      {/* Deletion Confirmation Modal */}
      {appToDelete && (
        <div className="p-4 mx-4 my-2 rounded-xl bg-rose-950/40 border border-rose-900/60 space-y-3">
          <div className="flex items-start space-x-2.5">
            <AlertIcon className="w-4 h-4 text-rose-400 shrink-0 mt-0.5" />
            <div>
              <p className="text-xs font-semibold text-rose-200">
                Delete Application "{appToDelete.name}"?
              </p>
              <p className="text-[11px] text-rose-300/80 mt-0.5">
                This will remove the application from the launcher and automatically clean up any group references.
              </p>
            </div>
          </div>

          <div className="flex items-center justify-end space-x-2 pt-1">
            <button
              type="button"
              onClick={() => setAppToDelete(null)}
              disabled={isDeleting}
              className="px-3 py-1 rounded-md text-xs font-medium text-neutral-300 hover:bg-neutral-800/60 transition-colors cursor-pointer"
            >
              Cancel
            </button>
            <button
              type="button"
              onClick={confirmDelete}
              disabled={isDeleting}
              className="px-3 py-1 rounded-md text-xs font-semibold bg-rose-600 hover:bg-rose-500 disabled:bg-rose-800 text-white transition-colors cursor-pointer shadow-sm"
            >
              {isDeleting ? "Deleting..." : "Confirm Delete"}
            </button>
          </div>
        </div>
      )}

      {/* Error Notice */}
      {error && (
        <div className="px-4 py-2 mx-4 mt-2 rounded-lg bg-rose-950/40 border border-rose-900/60 text-xs text-rose-300">
          {error}
        </div>
      )}

      {/* Applications List */}
      <div className="p-3 space-y-1 overflow-y-auto max-h-[290px]">
        {filtered.length === 0 ? (
          <div className="text-center py-8 text-xs text-neutral-500">
            {applications.length === 0
              ? "No applications registered yet."
              : "No applications match your filter."}
          </div>
        ) : (
          filtered.map((app) => (
            <div
              key={app.id}
              className="flex items-center justify-between px-3.5 py-2.5 rounded-lg bg-neutral-900/40 hover:bg-neutral-900/90 border border-neutral-800/60 transition-colors group"
            >
              <div className="flex items-center space-x-3 min-w-0 pr-2">
                <AppIcon className="w-4 h-4 text-blue-400 shrink-0" />
                <span className="font-mono text-xs px-2 py-0.5 rounded bg-neutral-800 border border-neutral-700 text-neutral-300 shrink-0">
                  {app.command}
                </span>
                <div className="min-w-0">
                  <div className="flex items-center space-x-2">
                    <span className="text-xs font-medium text-neutral-200 truncate">
                      {app.name}
                    </span>
                    {app.projectLaunch?.enabled && (
                      <span className="text-[9px] px-1.5 py-0.2 bg-blue-950/60 border border-blue-900 text-blue-300 rounded font-medium">
                        Project Context
                      </span>
                    )}
                  </div>
                  <span className="text-[10px] text-neutral-500 truncate block font-mono">
                    {app.executablePath}
                  </span>
                </div>
              </div>

              <div className="flex items-center space-x-1 shrink-0">
                <button
                  type="button"
                  onClick={() => onEdit(app)}
                  title="Edit application"
                  className="p-1.5 rounded-md text-neutral-400 hover:text-blue-400 hover:bg-neutral-800 transition-colors cursor-pointer"
                >
                  <EditIcon className="w-3.5 h-3.5" />
                </button>
                <button
                  type="button"
                  onClick={() => setAppToDelete(app)}
                  title="Delete application"
                  className="p-1.5 rounded-md text-neutral-400 hover:text-rose-400 hover:bg-neutral-800 transition-colors cursor-pointer"
                >
                  <TrashIcon className="w-3.5 h-3.5" />
                </button>
              </div>
            </div>
          ))
        )}
      </div>

      {/* Footer */}
      <div className="flex items-center justify-between px-5 py-2 border-t border-neutral-800/80 bg-neutral-950/90 text-[11px] text-neutral-500">
        <span>
          {applications.length} application{applications.length === 1 ? "" : "s"} registered
        </span>
        <span>esc to return</span>
      </div>
    </div>
  );
}
