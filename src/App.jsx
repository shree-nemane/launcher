import React, { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { useSuggestions } from "./hooks/useSuggestions";
import { useLauncher } from "./hooks/useLauncher";
import { SearchBar } from "./components/Launcher/SearchBar";
import { SuggestionsList } from "./components/Launcher/SuggestionsList";
import { ErrorNotice } from "./components/Launcher/ErrorNotice";
import { ShortcutFooter } from "./components/Launcher/ShortcutFooter";
import { ProjectFormView } from "./components/Config/ProjectFormView";
import { AppFormView } from "./components/Config/AppFormView";
import { GroupFormView } from "./components/Config/GroupFormView";
import { ManageProjectsView } from "./components/Config/ManageProjectsView";
import { ManageAppsView } from "./components/Config/ManageAppsView";
import { ManageGroupsView } from "./components/Config/ManageGroupsView";
import { HelpView } from "./components/Config/HelpView";
import { SettingsView } from "./components/Config/SettingsView";
import { HelpIcon, CloseIcon } from "./components/Launcher/Icons";
import { launcherService } from "./services/launcherService";
import "./App.css";

function App() {
  const [activeView, setActiveView] = useState("launcher");
  const [previousView, setPreviousView] = useState("launcher");
  const [editingProject, setEditingProject] = useState(null);
  const [editingApp, setEditingApp] = useState(null);
  const [editingGroup, setEditingGroup] = useState(null);
  const [transientInput, setTransientInput] = useState("");
  const { suggestions, reloadSuggestions } = useSuggestions(transientInput);

  const navigateTo = (view) => {
    setPreviousView(activeView);
    setActiveView(view);
  };

  // Global Escape key handling
  useEffect(() => {
    const handleGlobalKeyDown = (e) => {
      if (e.key === "Escape") {
        if (e.defaultPrevented) return;

        if (activeView === "launcher") {
          launcherService.hideLauncher();
        } else if (
          activeView === "addProject" ||
          activeView === "manageProjects" ||
          activeView === "addApp" ||
          activeView === "manageApps" ||
          activeView === "addGroup" ||
          activeView === "manageGroups" ||
          activeView === "help" ||
          activeView === "settings"
        ) {
          setActiveView("launcher");
        } else if (activeView === "editProject") {
          setActiveView(previousView === "manageProjects" ? "manageProjects" : "launcher");
        } else if (activeView === "editApp") {
          setActiveView(previousView === "manageApps" ? "manageApps" : "launcher");
        } else if (activeView === "editGroup") {
          setActiveView(previousView === "manageGroups" ? "manageGroups" : "launcher");
        }
      }
    };

    window.addEventListener("keydown", handleGlobalKeyDown);
    return () => window.removeEventListener("keydown", handleGlobalKeyDown);
  }, [activeView, previousView]);

  const {
    input,
    inputRef,
    selectedIndex,
    isExecuting,
    error,
    resetState,
    handleInputChange,
    handleClearInput,
    handleKeyDown,
    handleSelectSuggestion,
    handleHoverSuggestion,
    handleClose,
  } = useLauncher(suggestions, (view) => navigateTo(view));

  // Auto-focus search bar whenever returning to launcher view
  useEffect(() => {
    if (activeView === "launcher") {
      const timer = setTimeout(() => {
        inputRef.current?.focus();
      }, 30);
      return () => clearTimeout(timer);
    }
  }, [activeView, inputRef]);

  // Synchronize input for suggestions hook
  useEffect(() => {
    setTransientInput(input);
  }, [input]);

  const [shortcutWarning, setShortcutWarning] = useState(null);

  // Listen to native Tauri events (show/reset/navigate/shortcut status)
  useEffect(() => {
    let unlistenShow;
    let unlistenNav;
    let unlistenShortcut;

    async function registerListeners() {
      try {
        unlistenShow = await listen("launcher://show", () => {
          setActiveView("launcher");
          resetState();
          setTimeout(() => {
            inputRef.current?.focus();
          }, 30);
        });

        unlistenNav = await listen("launcher://navigate", (event) => {
          if (event.payload) {
            setActiveView(event.payload);
          }
        });

        unlistenShortcut = await listen("launcher://shortcut-status", (event) => {
          if (event.payload && (event.payload.status === "fallback" || event.payload.status === "failed")) {
            setShortcutWarning(event.payload.message);
          } else {
            setShortcutWarning(null);
          }
        });
      } catch (e) {
        console.warn("Event listener registration failed (running outside Tauri?):", e);
      }
    }

    registerListeners();

    return () => {
      if (unlistenShow) unlistenShow();
      if (unlistenNav) unlistenNav();
      if (unlistenShortcut) unlistenShortcut();
    };
  }, [resetState, inputRef]);

  const handleEditProject = (project) => {
    setEditingProject(project);
    navigateTo("editProject");
  };

  const handleEditApp = (app) => {
    setEditingApp(app);
    navigateTo("editApp");
  };

  const handleEditGroup = (group) => {
    setEditingGroup(group);
    navigateTo("editGroup");
  };

  const handleProjectSaved = () => {
    reloadSuggestions();
    setEditingProject(null);
    if (previousView === "manageProjects") {
      setActiveView("manageProjects");
    } else {
      setActiveView("launcher");
    }
  };

  const handleAppSaved = () => {
    reloadSuggestions();
    setEditingApp(null);
    if (previousView === "manageApps") {
      setActiveView("manageApps");
    } else {
      setActiveView("launcher");
    }
  };

  const handleGroupSaved = () => {
    reloadSuggestions();
    setEditingGroup(null);
    if (previousView === "manageGroups") {
      setActiveView("manageGroups");
    } else {
      setActiveView("launcher");
    }
  };

  const handleDataChanged = () => {
    reloadSuggestions();
  };

  return (
    <div className="min-h-screen w-screen flex items-start justify-center pt-4 px-3 bg-transparent select-none">
      <div className="w-[620px] rounded-2xl bg-neutral-950/95 border border-neutral-800/90 shadow-2xl shadow-black/90 backdrop-blur-xl overflow-hidden flex flex-col transition-all duration-150">
        {activeView === "launcher" && (
          <>
            <SearchBar
              input={input}
              inputRef={inputRef}
              isExecuting={isExecuting}
              hasSuggestions={suggestions.length > 0}
              onChange={handleInputChange}
              onKeyDown={handleKeyDown}
              onClear={handleClearInput}
              onOpenHelp={() => navigateTo("help")}
              onClose={handleClose}
            />

            <ErrorNotice error={error} />

            {shortcutWarning && (
              <div className="px-4 py-2 mx-3 my-1 rounded-lg bg-amber-950/50 border border-amber-900/60 text-xs text-amber-300 flex items-center justify-between">
                <span>{shortcutWarning}</span>
                <button
                  type="button"
                  onClick={() => setShortcutWarning(null)}
                  className="ml-2 text-amber-400 hover:text-amber-200 cursor-pointer font-bold"
                >
                  ×
                </button>
              </div>
            )}

            {!error && (
              <SuggestionsList
                suggestions={suggestions}
                selectedIndex={selectedIndex}
                onSelectSuggestion={handleSelectSuggestion}
                onHoverSuggestion={handleHoverSuggestion}
              />
            )}

            <ShortcutFooter onOpenHelp={() => navigateTo("help")} />
          </>
        )}

        {activeView === "addProject" && (
          <ProjectFormView
            initialData={null}
            onCancel={() => setActiveView("launcher")}
            onSuccess={handleProjectSaved}
          />
        )}

        {activeView === "editProject" && (
          <ProjectFormView
            initialData={editingProject}
            onCancel={() =>
              setActiveView(
                previousView === "manageProjects" ? "manageProjects" : "launcher"
              )
            }
            onSuccess={handleProjectSaved}
          />
        )}

        {activeView === "manageProjects" && (
          <ManageProjectsView
            onCancel={() => setActiveView("launcher")}
            onAddNew={() => navigateTo("addProject")}
            onEdit={handleEditProject}
            onDataChanged={handleDataChanged}
          />
        )}

        {activeView === "addApp" && (
          <AppFormView
            initialData={null}
            onCancel={() => setActiveView("launcher")}
            onSuccess={handleAppSaved}
          />
        )}

        {activeView === "editApp" && (
          <AppFormView
            initialData={editingApp}
            onCancel={() =>
              setActiveView(
                previousView === "manageApps" ? "manageApps" : "launcher"
              )
            }
            onSuccess={handleAppSaved}
          />
        )}

        {activeView === "manageApps" && (
          <ManageAppsView
            onCancel={() => setActiveView("launcher")}
            onAddNew={() => navigateTo("addApp")}
            onEdit={handleEditApp}
            onDataChanged={handleDataChanged}
          />
        )}

        {activeView === "addGroup" && (
          <GroupFormView
            initialData={null}
            onCancel={() => setActiveView("launcher")}
            onSuccess={handleGroupSaved}
          />
        )}

        {activeView === "editGroup" && (
          <GroupFormView
            initialData={editingGroup}
            onCancel={() =>
              setActiveView(
                previousView === "manageGroups" ? "manageGroups" : "launcher"
              )
            }
            onSuccess={handleGroupSaved}
          />
        )}

        {activeView === "manageGroups" && (
          <ManageGroupsView
            onCancel={() => setActiveView("launcher")}
            onAddNew={() => navigateTo("addGroup")}
            onEdit={handleEditGroup}
            onDataChanged={handleDataChanged}
          />
        )}

        {activeView === "help" && (
          <HelpView onCancel={() => setActiveView("launcher")} />
        )}

        {activeView === "settings" && (
          <SettingsView
            onCancel={() => setActiveView("launcher")}
            onSaved={() => {
              reloadSuggestions();
              setActiveView("launcher");
            }}
          />
        )}
      </div>
    </div>
  );
}

export default App;