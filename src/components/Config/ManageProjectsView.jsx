import React, { useState, useEffect, useCallback } from "react";
import { launcherService } from "../../services/launcherService";
import { FolderIcon, EditIcon, TrashIcon, SearchIcon, AlertIcon, CloseIcon } from "../Launcher/Icons";

export function ManageProjectsView({ onCancel, onAddNew, onEdit, onDataChanged }) {
  const [projects, setProjects] = useState([]);
  const [filterText, setFilterText] = useState("");
  const [projectToDelete, setProjectToDelete] = useState(null);
  const [isDeleting, setIsDeleting] = useState(false);
  const [error, setError] = useState(null);

  const loadProjects = useCallback(async () => {
    try {
      const list = await launcherService.getProjects();
      setProjects(list || []);
    } catch (err) {
      console.error("Failed to load projects:", err);
      setError("Failed to load projects list");
    }
  }, []);

  useEffect(() => {
    loadProjects();
  }, [loadProjects]);

  const handleKeyDown = (e) => {
    if (e.key === "Escape") {
      e.preventDefault();
      if (projectToDelete) {
        setProjectToDelete(null);
      } else {
        onCancel();
      }
    }
  };

  const confirmDelete = async () => {
    if (!projectToDelete || isDeleting) return;
    setIsDeleting(true);
    setError(null);
    try {
      await launcherService.deleteProject(projectToDelete.id);
      setProjectToDelete(null);
      await loadProjects();
      if (onDataChanged) {
        onDataChanged();
      }
    } catch (err) {
      let msg = "Failed to delete project";
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

  const filtered = projects.filter((p) => {
    const term = filterText.toLowerCase();
    return (
      p.name.toLowerCase().includes(term) ||
      p.command.toLowerCase().includes(term) ||
      p.path.toLowerCase().includes(term)
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
              Manage Projects
            </h2>
          </div>
        </div>

        <div className="flex items-center space-x-2">
          <button
            type="button"
            onClick={onAddNew}
            className="px-3 py-1 rounded-lg text-xs font-semibold bg-blue-600 hover:bg-blue-500 text-white transition-colors cursor-pointer shadow-sm shadow-blue-900/30"
          >
            + Add Project
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
          placeholder="Filter projects by name or command..."
          value={filterText}
          onChange={(e) => setFilterText(e.target.value)}
          className="w-full bg-transparent text-xs text-neutral-200 placeholder-neutral-500 focus:outline-none"
        />
      </div>

      {/* Deletion Confirmation Modal/Overlay */}
      {projectToDelete && (
        <div className="p-4 mx-4 my-2 rounded-xl bg-rose-950/40 border border-rose-900/60 space-y-3">
          <div className="flex items-start space-x-2.5">
            <AlertIcon className="w-4 h-4 text-rose-400 shrink-0 mt-0.5" />
            <div>
              <p className="text-xs font-semibold text-rose-200">
                Delete Project "{projectToDelete.name}"?
              </p>
              <p className="text-[11px] text-rose-300/80 mt-0.5">
                This will remove the project from the launcher registry. Your project folder on disk will not be touched.
              </p>
            </div>
          </div>

          <div className="flex items-center justify-end space-x-2 pt-1">
            <button
              type="button"
              onClick={() => setProjectToDelete(null)}
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

      {/* Project List */}
      <div className="p-3 space-y-1 overflow-y-auto max-h-[290px]">
        {filtered.length === 0 ? (
          <div className="text-center py-8 text-xs text-neutral-500">
            {projects.length === 0
              ? "No projects registered yet."
              : "No projects match your filter."}
          </div>
        ) : (
          filtered.map((proj) => (
            <div
              key={proj.id}
              className="flex items-center justify-between px-3.5 py-2.5 rounded-lg bg-neutral-900/40 hover:bg-neutral-900/90 border border-neutral-800/60 transition-colors group"
            >
              <div className="flex items-center space-x-3 min-w-0 pr-2">
                <FolderIcon className="w-4 h-4 text-amber-400 shrink-0" />
                <span className="font-mono text-xs px-2 py-0.5 rounded bg-neutral-800 border border-neutral-700 text-neutral-300 shrink-0">
                  {proj.command}
                </span>
                <div className="min-w-0">
                  <span className="text-xs font-medium text-neutral-200 block truncate">
                    {proj.name}
                  </span>
                  <span className="text-[10px] text-neutral-500 truncate block font-mono">
                    {proj.path}
                  </span>
                </div>
              </div>

              <div className="flex items-center space-x-1 shrink-0">
                <button
                  type="button"
                  onClick={() => onEdit(proj)}
                  title="Edit project"
                  className="p-1.5 rounded-md text-neutral-400 hover:text-blue-400 hover:bg-neutral-800 transition-colors cursor-pointer"
                >
                  <EditIcon className="w-3.5 h-3.5" />
                </button>
                <button
                  type="button"
                  onClick={() => setProjectToDelete(proj)}
                  title="Delete project"
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
        <span>{projects.length} project{projects.length === 1 ? "" : "s"} registered</span>
        <span>esc to return</span>
      </div>
    </div>
  );
}
