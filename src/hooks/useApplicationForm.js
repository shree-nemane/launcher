import { useState, useCallback, useEffect } from "react";
import { launcherService } from "../services/launcherService";

export function useApplicationForm({ initialData = null, onSuccess = null }) {
  const mode = initialData?.id ? "edit" : "create";

  const [formData, setFormData] = useState({
    name: initialData?.name || "",
    command: initialData?.command || "",
    executablePath: initialData?.executablePath || "",
    normalArguments: initialData?.normalLaunch?.arguments || [],
    projectLaunchEnabled: initialData?.projectLaunch?.enabled || false,
    projectArguments: initialData?.projectLaunch?.arguments || ["{PROJECT_PATH}"],
    workingDirectory: initialData?.workingDirectory || "",
  });

  const [errors, setErrors] = useState({});
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    if (initialData) {
      setFormData({
        name: initialData.name || "",
        command: initialData.command || "",
        executablePath: initialData.executablePath || "",
        normalArguments: initialData.normalLaunch?.arguments || [],
        projectLaunchEnabled: initialData.projectLaunch?.enabled || false,
        projectArguments: initialData.projectLaunch?.arguments || ["{PROJECT_PATH}"],
        workingDirectory: initialData.workingDirectory || "",
      });
    }
  }, [initialData]);

  const applyPreset = useCallback((preset) => {
    setFormData({
      name: preset.name || "",
      command: preset.command || "",
      executablePath: preset.executablePath || "",
      normalArguments: preset.normalArguments || [],
      projectLaunchEnabled: preset.projectLaunchEnabled || false,
      projectArguments: preset.projectArguments || [],
      workingDirectory: preset.workingDirectory || "",
    });
    setErrors({});
  }, []);

  const handleChange = useCallback((field, value) => {
    setFormData((prev) => {
      const next = { ...prev, [field]: value };

      // If user enables project launch and arguments are empty, choose smart default based on app type
      if (field === "projectLaunchEnabled" && value && (!prev.projectArguments || prev.projectArguments.length === 0)) {
        const lowerExe = prev.executablePath.toLowerCase();
        if (lowerExe.includes("pwsh") || lowerExe.includes("powershell")) {
          next.projectArguments = ["-NoExit"];
        } else if (lowerExe.includes("cmd")) {
          next.projectArguments = ["/k"];
        } else if (lowerExe.includes("chrome") || lowerExe.includes("msedge") || lowerExe.includes("brave") || lowerExe.includes("firefox")) {
          next.projectArguments = ["{PROJECT_URL}"];
        } else {
          next.projectArguments = ["{PROJECT_PATH}"];
        }
      }

      return next;
    });
    setErrors((prev) => ({ ...prev, [field]: null, general: null }));
  }, []);

  const handleBrowseExecutable = useCallback(async () => {
    try {
      const selected = await launcherService.pickExecutable();
      if (selected) {
        setFormData((prev) => ({ ...prev, executablePath: selected }));
        setErrors((prev) => ({ ...prev, executablePath: null, general: null }));
      }
    } catch (err) {
      console.error("Executable picker error:", err);
    }
  }, []);

  const validateFrontend = () => {
    const errs = {};
    if (!formData.name.trim()) {
      errs.name = "Application name is required";
    }

    const cmd = formData.command.trim();
    if (!cmd) {
      errs.command = "Command is required";
    } else if (!cmd.startsWith("/")) {
      errs.command = "Command must start with '/'";
    } else if (cmd === "//") {
      errs.command = "Command cannot be '//' (reserved for default group)";
    } else if (cmd.includes(" ")) {
      errs.command = "Command cannot contain spaces";
    }

    if (!formData.executablePath.trim()) {
      errs.executablePath = "Executable path is required";
    }

    return errs;
  };

  const handleSave = useCallback(async () => {
    if (isSaving) return;

    const frontendErrors = validateFrontend();
    if (Object.keys(frontendErrors).length > 0) {
      setErrors(frontendErrors);
      return;
    }

    setIsSaving(true);
    setErrors({});

    try {
      const appPayload = {
        id: initialData?.id || "",
        name: formData.name.trim(),
        command: formData.command.trim().toLowerCase(),
        executablePath: formData.executablePath.trim(),
        normalLaunch: {
          arguments: formData.normalArguments.filter((a) => a.trim() !== ""),
        },
        projectLaunch: formData.projectLaunchEnabled
          ? {
              enabled: true,
              arguments: formData.projectArguments.filter((a) => a.trim() !== ""),
            }
          : null,
        workingDirectory: formData.workingDirectory.trim()
          ? formData.workingDirectory.trim()
          : null,
        icon: initialData?.icon || null,
        createdAt: initialData?.createdAt || "",
        updatedAt: initialData?.updatedAt || "",
      };

      if (mode === "edit") {
        await launcherService.updateApplication(appPayload);
      } else {
        await launcherService.createApplication(appPayload);
      }

      if (onSuccess) {
        onSuccess();
      }
    } catch (err) {
      let msg = `Failed to ${mode === "edit" ? "update" : "save"} application`;
      if (err && typeof err === "object") {
        msg = err.message || JSON.stringify(err);
      } else if (typeof err === "string") {
        msg = err;
      }
      setErrors({ general: msg });
    } finally {
      setIsSaving(false);
    }
  }, [formData, isSaving, initialData, mode, onSuccess]);

  return {
    mode,
    formData,
    errors,
    isSaving,
    handleChange,
    applyPreset,
    handleBrowseExecutable,
    handleSave,
  };
}
