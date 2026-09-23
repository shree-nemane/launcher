import { useState, useCallback, useEffect, useRef } from "react";
import { launcherService } from "../services/launcherService";
import { applySuggestionToInput } from "../utils/commandInput";

export function useLauncher(suggestions = [], onNavigate = null, controlledInput = null, setControlledInput = null) {
  const [internalInput, setInternalInput] = useState("");
  const input = setControlledInput ? controlledInput : internalInput;
  const setInput = setControlledInput || setInternalInput;
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [isExecuting, setIsExecuting] = useState(false);
  const [error, setError] = useState(null);
  const inputRef = useRef(null);

  // Clamp selected index within available suggestions
  useEffect(() => {
    if (suggestions.length === 0) {
      setSelectedIndex(0);
    } else if (selectedIndex >= suggestions.length) {
      setSelectedIndex(Math.max(0, suggestions.length - 1));
    }
  }, [suggestions, selectedIndex]);

  // Keep input focused automatically
  useEffect(() => {
    inputRef.current?.focus();
  }, []);

  const resetState = useCallback(() => {
    setInput("");
    setError(null);
    setSelectedIndex(0);
    setTimeout(() => {
      inputRef.current?.focus();
    }, 0);
  }, []);

  const handleInputChange = useCallback((e) => {
    setInput(e.target.value);
    setError(null); // Clear error on new typing
  }, []);

  const handleSelectSuggestion = useCallback(
    (suggestion) => {
      if (isExecuting) return;

      if (suggestion.kind === "system" && onNavigate) {
        const cmdName = suggestion.command.replace(/^\//, "");
        if (cmdName === "add-project") {
          setInput("");
          setError(null);
          onNavigate("addProject");
          return;
        }
        if (cmdName === "add-app") {
          setInput("");
          setError(null);
          onNavigate("addApp");
          return;
        }
        if (cmdName === "manage-projects") {
          setInput("");
          setError(null);
          onNavigate("manageProjects");
          return;
        }
        if (cmdName === "manage-apps") {
          setInput("");
          setError(null);
          onNavigate("manageApps");
          return;
        }
        if (cmdName === "add-group") {
          setInput("");
          setError(null);
          onNavigate("addGroup");
          return;
        }
        if (cmdName === "manage-groups") {
          setInput("");
          setError(null);
          onNavigate("manageGroups");
          return;
        }
        if (cmdName === "help") {
          setInput("");
          setError(null);
          onNavigate("help");
          return;
        }
        if (cmdName === "settings" || cmdName === "config") {
          setInput("");
          setError(null);
          onNavigate("settings");
          return;
        }
      }

      const updated = applySuggestionToInput(input, suggestion.command);
      setInput(updated);
      setError(null);
      inputRef.current?.focus();
    },
    [input, onNavigate]
  );

  const handleHoverSuggestion = useCallback((index) => {
    setSelectedIndex(index);
  }, []);

  const execute = useCallback(
    async (overrideInput) => {
      if (isExecuting) return;

      const rawCmd = (overrideInput ?? input).trim();
      if (!rawCmd) return;

      let cmd = rawCmd;
      if (cmd.startsWith("/") && !cmd.startsWith("//")) {
        const stripped = cmd.slice(1);
        if (
          [
            "manage-projects",
            "manage-apps",
            "manage-groups",
            "add-project",
            "add-app",
            "add-group",
            "settings",
            "config",
            "help",
          ].includes(stripped)
        ) {
          cmd = stripped;
        }
      }

      setIsExecuting(true);
      setError(null);

      try {
        // Plan first to distinguish UI system commands from OS launch execution
        const plan = await launcherService.planCommand(cmd);

        if (plan.type === "system") {
          if (plan.command === "add-project" && onNavigate) {
            setInput("");
            onNavigate("addProject");
            return;
          }
          if (plan.command === "add-app" && onNavigate) {
            setInput("");
            onNavigate("addApp");
            return;
          }
          if (plan.command === "manage-projects" && onNavigate) {
            setInput("");
            onNavigate("manageProjects");
            return;
          }
          if (plan.command === "manage-apps" && onNavigate) {
            setInput("");
            onNavigate("manageApps");
            return;
          }
          if (plan.command === "add-group" && onNavigate) {
            setInput("");
            onNavigate("addGroup");
            return;
          }
          if (plan.command === "manage-groups" && onNavigate) {
            setInput("");
            onNavigate("manageGroups");
            return;
          }
          if (plan.command === "help" && onNavigate) {
            setInput("");
            onNavigate("help");
            return;
          }
          if (plan.command === "settings" && onNavigate) {
            setInput("");
            onNavigate("settings");
            return;
          }
        }

        const result = await launcherService.executeCommand(cmd);

        if (result.success) {
          setInput("");
          await launcherService.hideLauncher();
        } else {
          // Action level failure
          const failedAction = result.actionResults?.find((a) => !a.success);
          const errorMsg =
            failedAction?.error ||
            `Action '${failedAction?.name || "Command"}' failed to launch`;
          setError(errorMsg);
        }
      } catch (err) {
        // Pipeline / Validation / Resolution error
        let msg = "An unexpected error occurred";
        if (err && typeof err === "object") {
          msg = err.message || JSON.stringify(err);
        } else if (typeof err === "string") {
          msg = err;
        }
        setError(msg);
      } finally {
        setIsExecuting(false);
        inputRef.current?.focus();
      }
    },
    [input, isExecuting, onNavigate]
  );

  const handleKeyDown = useCallback(
    (e) => {
      if (isExecuting) return;

      if (e.key === "Escape") {
        e.preventDefault();
        launcherService.hideLauncher();
        return;
      }

      if (e.key === "ArrowDown") {
        e.preventDefault();
        if (suggestions.length > 0) {
          setSelectedIndex((prev) => (prev + 1) % suggestions.length);
        }
        return;
      }

      if (e.key === "ArrowUp") {
        e.preventDefault();
        if (suggestions.length > 0) {
          setSelectedIndex((prev) =>
            prev <= 0 ? suggestions.length - 1 : prev - 1
          );
        }
        return;
      }

      if (e.key === "Tab") {
        e.preventDefault();
        if (suggestions.length > 0 && suggestions[selectedIndex]) {
          handleSelectSuggestion(suggestions[selectedIndex]);
        }
        return;
      }

      if (e.key === "Enter") {
        e.preventDefault();

        const selected = suggestions.length > 0 ? suggestions[selectedIndex] : null;
        if (selected) {
          if (selected.kind === "system") {
            handleSelectSuggestion(selected);
            return;
          }

          const tokens = input.trim().split(/\s+/).filter(Boolean);
          const isProjectLaunchWithApp =
            input.endsWith(" ") ||
            (tokens.length === 1 &&
              !tokens[0].startsWith("/") &&
              tokens[0] !== "//" &&
              (selected.kind === "app" || selected.kind === "group"));

          if (tokens.length <= 1 && !isProjectLaunchWithApp) {
            execute(selected.command);
            return;
          } else {
            const updated = applySuggestionToInput(input, selected.command);
            execute(updated);
            return;
          }
        }

        execute();
      }
    },
    [isExecuting, suggestions, selectedIndex, input, handleSelectSuggestion, execute]
  );

  const handleClearInput = useCallback(() => {
    setInput("");
    setError(null);
    inputRef.current?.focus();
  }, []);

  const handleClose = useCallback(() => {
    launcherService.hideLauncher();
  }, []);

  return {
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
  };
}
