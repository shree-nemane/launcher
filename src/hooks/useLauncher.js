import { useState, useCallback, useEffect, useRef } from "react";
import { launcherService } from "../services/launcherService";
import { applySuggestionToInput } from "../utils/commandInput";

export function useLauncher(suggestions = [], onNavigate = null) {
  const [input, setInput] = useState("");
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
        if (suggestion.command === "add-project") {
          setInput("");
          setError(null);
          onNavigate("addProject");
          return;
        }
        if (suggestion.command === "add-app") {
          setInput("");
          setError(null);
          onNavigate("addApp");
          return;
        }
        if (suggestion.command === "manage-projects") {
          setInput("");
          setError(null);
          onNavigate("manageProjects");
          return;
        }
        if (suggestion.command === "manage-apps") {
          setInput("");
          setError(null);
          onNavigate("manageApps");
          return;
        }
        if (suggestion.command === "add-group") {
          setInput("");
          setError(null);
          onNavigate("addGroup");
          return;
        }
        if (suggestion.command === "manage-groups") {
          setInput("");
          setError(null);
          onNavigate("manageGroups");
          return;
        }
        if (suggestion.command === "help") {
          setInput("");
          setError(null);
          onNavigate("help");
          return;
        }
        if (suggestion.command === "settings") {
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

      const cmd = (overrideInput ?? input).trim();
      if (!cmd) return;

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
        execute();
      }
    },
    [isExecuting, suggestions, selectedIndex, handleSelectSuggestion, execute]
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
