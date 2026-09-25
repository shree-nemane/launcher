import { useState, useCallback, useEffect, useRef } from "react";
import { launcherService } from "../services/launcherService";
import { applySuggestionToInput } from "../utils/commandInput";

export function useLauncher(suggestions = [], onNavigate = null, controlledInput = null, setControlledInput = null) {
  const [internalInput, setInternalInput] = useState("");
  const input = setControlledInput ? controlledInput : internalInput;
  const setInput = setControlledInput || setInternalInput;
  const [selectedIndex, setSelectedIndex] = useState(-1);
  const [isExecuting, setIsExecuting] = useState(false);
  const isExecutingRef = useRef(false);
  const [error, setError] = useState(null);
  const inputRef = useRef(null);

  // Clamp selected index within available suggestions
  useEffect(() => {
    if (suggestions.length === 0) {
      setSelectedIndex(-1);
    } else if (selectedIndex >= suggestions.length) {
      setSelectedIndex(suggestions.length - 1);
    }
  }, [suggestions, selectedIndex]);

  // Keep input focused automatically
  useEffect(() => {
    inputRef.current?.focus();
  }, []);

  const resetState = useCallback(() => {
    setInput("");
    setError(null);
    setSelectedIndex(-1);
    setTimeout(() => {
      inputRef.current?.focus();
    }, 0);
  }, [setInput]);

  const handleInputChange = useCallback((e) => {
    setInput(e.target.value);
    setError(null); // Clear error on new typing
    setSelectedIndex(-1);
  }, [setInput]);

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
      if (isExecutingRef.current) return;

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

      isExecutingRef.current = true;
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
        isExecutingRef.current = false;
        setIsExecuting(false);
        inputRef.current?.focus();
      }
    },
    [input, isExecuting, onNavigate]
  );

  const handleKeyDown = useCallback(
    (e) => {
      if (isExecutingRef.current || isExecuting) return;

      if (e.key === "ArrowDown") {
        e.preventDefault();
        if (suggestions.length > 0) {
          setSelectedIndex((prev) => (prev < suggestions.length - 1 ? prev + 1 : 0));
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
        const targetIndex = selectedIndex >= 0 ? selectedIndex : 0;
        if (suggestions.length > 0 && suggestions[targetIndex]) {
          handleSelectSuggestion(suggestions[targetIndex]);
          setSelectedIndex(-1);
        }
        return;
      }

      if (e.key === "Enter") {
        e.preventDefault();

        const trimmed = input.trim();

        // If input is empty and an item was selected with arrow keys, select it
        if (!trimmed) {
          if (selectedIndex >= 0 && suggestions[selectedIndex]) {
            handleSelectSuggestion(suggestions[selectedIndex]);
          }
          return;
        }

        // If user explicitly navigated down to a suggestion with arrow keys
        if (selectedIndex >= 0 && suggestions[selectedIndex]) {
          const selected = suggestions[selectedIndex];
          if (selected.kind === "system") {
            handleSelectSuggestion(selected);
            return;
          }
          // Autocomplete the suggestion into the input text box so it is explicitly typed
          const updated = applySuggestionToInput(input, selected.command);
          if (updated.trim() !== trimmed) {
            setInput(updated);
            setSelectedIndex(-1);
            return;
          }
        }

        // Execute ONLY what is explicitly typed in the text box
        execute(trimmed);
        return;
      }
    },
    [isExecuting, suggestions, selectedIndex, input, handleSelectSuggestion, execute]
  );

  const handleClearInput = useCallback(() => {
    setInput("");
    setError(null);
    setSelectedIndex(-1);
    inputRef.current?.focus();
  }, [setInput]);

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
