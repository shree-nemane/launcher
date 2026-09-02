import React, { useState, useEffect, useCallback } from "react";
import { launcherService } from "../../services/launcherService";
import { GroupIcon, AppIcon, EditIcon, TrashIcon, SearchIcon, AlertIcon, CloseIcon } from "../Launcher/Icons";

export function ManageGroupsView({ onCancel, onAddNew, onEdit, onDataChanged }) {
  const [groups, setGroups] = useState([]);
  const [applications, setApplications] = useState([]);
  const [defaultGroupId, setDefaultGroupId] = useState(null);
  const [filterText, setFilterText] = useState("");
  const [groupToDelete, setGroupToDelete] = useState(null);
  const [isDeleting, setIsDeleting] = useState(false);
  const [error, setError] = useState(null);

  const loadData = useCallback(async () => {
    setError(null);
    try {
      const [groupsResult, appsResult, settingsResult] = await Promise.allSettled([
        launcherService.getGroups(),
        launcherService.getApplications(),
        launcherService.getSettings(),
      ]);

      if (groupsResult.status === "fulfilled") {
        setGroups(groupsResult.value || []);
      } else {
        console.error("Failed to load groups:", groupsResult.reason);
      }

      if (appsResult.status === "fulfilled") {
        setApplications(appsResult.value || []);
      } else {
        console.error("Failed to load apps:", appsResult.reason);
      }

      if (settingsResult.status === "fulfilled") {
        setDefaultGroupId(settingsResult.value?.defaultApplicationGroupId || null);
      } else {
        console.error("Failed to load settings:", settingsResult.reason);
      }

      // If groups failed specifically, report the error
      if (groupsResult.status === "rejected") {
        const reason = groupsResult.reason;
        const msg = typeof reason === "string" ? reason : reason?.message || JSON.stringify(reason);
        setError(`Failed to load application groups: ${msg}`);
      }
    } catch (err) {
      console.error("Failed to load groups data:", err);
      let msg = "Failed to load application groups list";
      if (err && typeof err === "object") {
        msg = err.message || JSON.stringify(err);
      } else if (typeof err === "string") {
        msg = err;
      }
      setError(msg);
    }
  }, []);

  useEffect(() => {
    loadData();
  }, [loadData]);

  const handleKeyDown = (e) => {
    if (e.key === "Escape") {
      e.preventDefault();
      if (groupToDelete) {
        setGroupToDelete(null);
      } else {
        onCancel();
      }
    }
  };

  const handleToggleDefault = async (groupId) => {
    setError(null);
    try {
      const newDefault = defaultGroupId === groupId ? null : groupId;
      await launcherService.setDefaultGroup(newDefault);
      setDefaultGroupId(newDefault);
      if (onDataChanged) {
        onDataChanged();
      }
    } catch (err) {
      let msg = "Failed to update default group";
      if (err && typeof err === "object") {
        msg = err.message || JSON.stringify(err);
      } else if (typeof err === "string") {
        msg = err;
      }
      setError(msg);
    }
  };

  const confirmDelete = async () => {
    if (!groupToDelete || isDeleting) return;
    setIsDeleting(true);
    setError(null);
    try {
      await launcherService.deleteGroup(groupToDelete.id);
      setGroupToDelete(null);
      await loadData();
      if (onDataChanged) {
        onDataChanged();
      }
    } catch (err) {
      let msg = "Failed to delete application group";
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

  const appMap = new Map(applications.map((a) => [a.id, a]));

  const filtered = groups.filter((g) => {
    const term = filterText.toLowerCase();
    return g.name.toLowerCase().includes(term);
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
              Manage Application Groups
            </h2>
          </div>
        </div>

        <div className="flex items-center space-x-2">
          <button
            type="button"
            onClick={onAddNew}
            className="px-3 py-1 rounded-lg text-xs font-semibold bg-blue-600 hover:bg-blue-500 text-white transition-colors cursor-pointer shadow-sm shadow-blue-900/30"
          >
            + Add Group
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
          placeholder="Filter groups by name..."
          value={filterText}
          onChange={(e) => setFilterText(e.target.value)}
          className="w-full bg-transparent text-xs text-neutral-200 placeholder-neutral-500 focus:outline-none"
        />
      </div>

      {/* Deletion Confirmation Modal */}
      {groupToDelete && (
        <div className="p-4 mx-4 my-2 rounded-xl bg-rose-950/40 border border-rose-900/60 space-y-3">
          <div className="flex items-start space-x-2.5">
            <AlertIcon className="w-4 h-4 text-rose-400 shrink-0 mt-0.5" />
            <div>
              <p className="text-xs font-semibold text-rose-200">
                Delete Group "{groupToDelete.name}"?
              </p>
              <p className="text-[11px] text-rose-300/80 mt-0.5">
                This will remove the group from the launcher. Your individual applications will not be modified.
              </p>
            </div>
          </div>

          <div className="flex items-center justify-end space-x-2 pt-1">
            <button
              type="button"
              onClick={() => setGroupToDelete(null)}
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

      {/* Groups List */}
      <div className="p-3 space-y-2 overflow-y-auto max-h-[290px]">
        {filtered.length === 0 ? (
          <div className="text-center py-8 text-xs text-neutral-500">
            {groups.length === 0
              ? "No application groups created yet. Click '+ Add Group' to create one for // workspace launch."
              : "No application groups match your filter."}
          </div>
        ) : (
          filtered.map((group) => {
            const isDefault = defaultGroupId === group.id;
            const orderList =
              group.executionOrder && group.executionOrder.length > 0
                ? group.executionOrder
                : group.applications;

            return (
              <div
                key={group.id}
                className={`p-3 rounded-xl border transition-all ${
                  isDefault
                    ? "bg-blue-950/20 border-blue-600/50 shadow-sm shadow-blue-950/50"
                    : "bg-neutral-900/40 hover:bg-neutral-900/90 border-neutral-800/60"
                }`}
              >
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-2.5 min-w-0 pr-2">
                    <GroupIcon
                      className={`w-4 h-4 shrink-0 ${
                        isDefault ? "text-blue-400" : "text-neutral-400"
                      }`}
                    />
                    <div className="min-w-0">
                      <div className="flex items-center space-x-2">
                        <span className="text-xs font-semibold text-neutral-100 truncate">
                          {group.name}
                        </span>
                        {isDefault && (
                          <span className="text-[10px] px-2 py-0.5 bg-blue-600 text-white font-mono font-medium rounded shadow-sm">
                            // Default Group
                          </span>
                        )}
                      </div>
                    </div>
                  </div>

                  <div className="flex items-center space-x-1.5 shrink-0">
                    <button
                      type="button"
                      onClick={() => handleToggleDefault(group.id)}
                      title={isDefault ? "Currently assigned to //" : "Set as default for // command"}
                      className={`px-2.5 py-1 rounded-md text-[11px] font-medium transition-colors cursor-pointer ${
                        isDefault
                          ? "bg-blue-600/20 text-blue-300 border border-blue-500/40 hover:bg-blue-600/30"
                          : "bg-neutral-800 text-neutral-300 hover:text-white hover:bg-neutral-700"
                      }`}
                    >
                      {isDefault ? "★ Active //" : "Set as //"}
                    </button>

                    <button
                      type="button"
                      onClick={() => onEdit(group)}
                      title="Edit group"
                      className="p-1.5 rounded-md text-neutral-400 hover:text-blue-400 hover:bg-neutral-800 transition-colors cursor-pointer"
                    >
                      <EditIcon className="w-3.5 h-3.5" />
                    </button>
                    <button
                      type="button"
                      onClick={() => setGroupToDelete(group)}
                      title="Delete group"
                      className="p-1.5 rounded-md text-neutral-400 hover:text-rose-400 hover:bg-neutral-800 transition-colors cursor-pointer"
                    >
                      <TrashIcon className="w-3.5 h-3.5" />
                    </button>
                  </div>
                </div>

                {/* Included Apps */}
                <div className="mt-2.5 pt-2 border-t border-neutral-800/60 flex flex-wrap items-center gap-1.5">
                  <span className="text-[10px] text-neutral-500 uppercase tracking-wider font-semibold mr-1">
                    Order:
                  </span>
                  {orderList.length === 0 ? (
                    <span className="text-[11px] text-neutral-600 italic">
                      No applications assigned
                    </span>
                  ) : (
                    orderList.map((appId, index) => {
                      const app = appMap.get(appId);
                      return (
                        <span
                          key={appId}
                          className="inline-flex items-center space-x-1.5 px-2 py-0.5 rounded-md bg-neutral-800/80 border border-neutral-700/60 text-[11px] text-neutral-300"
                        >
                          <span className="text-[9px] text-neutral-500 font-mono">
                            {index + 1}.
                          </span>
                          <span className="font-mono text-blue-400 font-medium">
                            {app?.command || appId}
                          </span>
                          <span className="text-neutral-200">
                            {app?.name || "Unknown"}
                          </span>
                        </span>
                      );
                    })
                  )}
                </div>
              </div>
            );
          })
        )}
      </div>

      {/* Footer */}
      <div className="flex items-center justify-between px-5 py-2 border-t border-neutral-800/80 bg-neutral-950/90 text-[11px] text-neutral-500">
        <span>
          {groups.length} group{groups.length === 1 ? "" : "s"} &bull; Type{" "}
          <span className="font-mono text-neutral-300">//</span> to launch default
        </span>
        <span>esc to return</span>
      </div>
    </div>
  );
}
