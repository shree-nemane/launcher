import React, { useState, useEffect, useRef, Suspense, lazy } from "react";
import { listen } from "@tauri-apps/api/event";
import { useSuggestions } from "./hooks/useSuggestions";
import { useLauncher } from "./hooks/useLauncher";
import { SearchBar } from "./components/Launcher/SearchBar";
import { SuggestionsList } from "./components/Launcher/SuggestionsList";
import { ErrorNotice } from "./components/Launcher/ErrorNotice";
import { ShortcutFooter } from "./components/Launcher/ShortcutFooter";
import { HelpIcon, CloseIcon } from "./components/Launcher/Icons";
import { launcherService } from "./services/launcherService";
import "./App.css";

const ProjectFormView = lazy(() =>
  import("./components/Config/ProjectFormView").then((m) => ({
    default: m.ProjectFormView,
  }))
);
const AppFormView = lazy(() =>
  import("./components/Config/AppFormView").then((m) => ({
    default: m.AppFormView,
  }))
);
const GroupFormView = lazy(() =>
  import("./components/Config/GroupFormView").then((m) => ({
    default: m.GroupFormView,
  }))
);
const ManageProjectsView = lazy(() =>
  import("./components/Config/ManageProjectsView").then((m) => ({
    default: m.ManageProjectsView,
  }))
);
const ManageAppsView = lazy(() =>
  import("./components/Config/ManageAppsView").then((m) => ({
    default: m.ManageAppsView,
  }))
);
const ManageGroupsView = lazy(() =>
  import("./components/Config/ManageGroupsView").then((m) => ({
    default: m.ManageGroupsView,
  }))
);
const HelpView = lazy(() =>
  import("./components/Config/HelpView").then((m) => ({
    default: m.HelpView,
  }))
);
const SettingsView = lazy(() =>
  import("./components/Config/SettingsView").then((m) => ({
    default: m.SettingsView,
  }))
);

function App() {
  const [activeView, setActiveView] = useState("launcher");
  const [previousView, setPreviousView] = useState("launcher");
  const [isWindowVisible, setIsWindowVisible] = useState(true);
  const [editingProject, setEditingProject] = useState(null);
  const [editingApp, setEditingApp] = useState(null);
  const [editingGroup, setEditingGroup] = useState(null);
  const [input, setInput] = useState("");
  const { suggestions, reloadSuggestions } = useSuggestions(input);

  const activeViewRef = useRef(activeView);
  activeViewRef.current = activeView;
  const lastShowTimeRef = useRef(Date.now());

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
          setActiveView(
            previousView === "manageProjects" ? "manageProjects" : "launcher"
          );
        } else if (activeView === "editApp") {
          setActiveView(
            previousView === "manageApps" ? "manageApps" : "launcher"
          );
        } else if (activeView === "editGroup") {
          setActiveView(
            previousView === "manageGroups" ? "manageGroups" : "launcher"
          );
        }
      }
    };

    window.addEventListener("keydown", handleGlobalKeyDown);
    return () => window.removeEventListener("keydown", handleGlobalKeyDown);
  }, [activeView, previousView]);

  const {
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
  } = useLauncher(suggestions, navigateTo, input, setInput);

  // Auto-focus search bar whenever returning to launcher view
  useEffect(() => {
    if (activeView === "launcher") {
      const timer = setTimeout(() => {
        inputRef.current?.focus();
        launcherService.signalLauncherReady();
      }, 30);
      return () => clearTimeout(timer);
    }
  }, [activeView, inputRef]);

  const [shortcutWarning, setShortcutWarning] = useState(null);

  // Listen to native Tauri events (show/hide/reset/navigate/shortcut status)
  useEffect(() => {
    let isCancelled = false;
    let cleanupFns = [];

    async function registerListeners() {
      try {
        const promises = [
          listen("launcher://show", () => {
            lastShowTimeRef.current = Date.now();
            setIsWindowVisible(true);
            setActiveView("launcher");
            resetState();
            setTimeout(() => {
              inputRef.current?.focus();
            }, 30);
          }),
          listen("launcher://hide", () => {
            setIsWindowVisible(false);
            resetState();
            if (typeof window !== "undefined" && typeof window.gc === "function") {
              try {
                window.gc();
              } catch (_) {}
            }
          }),
          listen("launcher://navigate", (event) => {
            if (event.payload) {
              lastShowTimeRef.current = Date.now();
              setIsWindowVisible(true);
              setActiveView(event.payload);
            }
          }),
          listen("launcher://shortcut-status", (event) => {
            if (event.payload && (event.payload.status === "fallback" || event.payload.status === "failed")) {
              setShortcutWarning(event.payload.message);
            } else {
              setShortcutWarning(null);
            }
          }),
          listen("launcher://focus-changed", (event) => {
            if (
              !event.payload &&
              activeViewRef.current === "launcher" &&
              Date.now() - lastShowTimeRef.current > 100
            ) {
              launcherService.hideLauncher();
            }
          }),
        ];

        const resolvedFns = await Promise.all(promises);
        if (isCancelled) {
          resolvedFns.forEach((unlisten) => unlisten());
        } else {
          cleanupFns = resolvedFns;
        }
      } catch (e) {
        console.warn("Event listener registration failed (running outside Tauri?):", e);
      }
    }

    registerListeners();

    return () => {
      isCancelled = true;
      cleanupFns.forEach((unlisten) => unlisten());
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
    <div className="min-h-screen w-screen flex items-start justify-center pt-2 px-2 bg-transparent select-none">
      <div className="w-[620px] rounded-2xl bg-neutral-950 border border-neutral-800/90 overflow-hidden flex flex-col transition-all duration-150">
        {!isWindowVisible ? null : (
          <>
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
                  onOpenManage={() => navigateTo("manageProjects")}
                  onOpenSettings={() => navigateTo("settings")}
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

                <ShortcutFooter
                  onOpenManage={() => navigateTo("manageProjects")}
                  onOpenSettings={() => navigateTo("settings")}
                  onOpenHelp={() => navigateTo("help")}
                />
              </>
            )}

            <Suspense
              fallback={
                <div className="h-48 flex items-center justify-center text-xs text-neutral-500">
                  Loading...
                </div>
              }
            >
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
                  onNavigate={navigateTo}
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
                  onNavigate={navigateTo}
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
                  onNavigate={navigateTo}
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
                  onNavigate={navigateTo}
                />
              )}
            </Suspense>
          </>
        )}
      </div>
    </div>
  );
}

export default App;