import { useState, useCallback, useEffect } from "react";
import { launcherService } from "../services/launcherService";

export function useProjectForm({ initialData = null, onSuccess = null }) {
  const mode = initialData?.id ? "edit" : "create";

  const [formData, setFormData] = useState({
    name: initialData?.name || "",
    command: initialData?.command || "",
    path: initialData?.path || "",
    url: initialData?.url || "",
    workingDirectory: initialData?.workingDirectory || "",
    runCommands: initialData?.runCommands || [],
  });

  const [errors, setErrors] = useState({});
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    if (initialData) {
      setFormData({
        name: initialData.name || "",
        command: initialData.command || "",
        path: initialData.path || "",
        url: initialData.url || "",
        workingDirectory: initialData.workingDirectory || "",
        runCommands: initialData.runCommands || [],
      });
    }
  }, [initialData]);

  const handleChange = useCallback((field, value) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
    setErrors((prev) => ({ ...prev, [field]: null, general: null }));
  }, []);

  const handleBrowseFolder = useCallback(async () => {
    try {
      const selected = await launcherService.pickFolder();
      if (selected) {
        setFormData((prev) => ({ ...prev, path: selected }));
        setErrors((prev) => ({ ...prev, path: null, general: null }));
      }
    } catch (err) {
      console.error("Folder picker error:", err);
    }
  }, []);

  const validateFrontend = () => {
    const errs = {};
    if (!formData.name.trim()) {
      errs.name = "Project name is required";
    }

    const cmd = formData.command.trim();
    if (!cmd) {
      errs.command = "Command is required";
    } else if (cmd.includes(" ")) {
      errs.command = "Command cannot contain spaces";
    } else if (cmd.includes("/")) {
      errs.command = "Project command cannot contain '/'";
    }

    if (!formData.path.trim()) {
      errs.path = "Project folder path is required";
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
      const projectPayload = {
        id: initialData?.id || "",
        name: formData.name.trim(),
        command: formData.command.trim().toLowerCase(),
        path: formData.path.trim(),
        url: formData.url.trim() ? formData.url.trim() : null,
        workingDirectory: formData.workingDirectory.trim()
          ? formData.workingDirectory.trim()
          : null,
        runCommands: formData.runCommands.filter(
          (c) => c.name.trim() && c.command.trim()
        ),
        createdAt: initialData?.createdAt || "",
        updatedAt: initialData?.updatedAt || "",
      };

      if (mode === "edit") {
        await launcherService.updateProject(projectPayload);
      } else {
        await launcherService.createProject(projectPayload);
      }

      if (onSuccess) {
        onSuccess();
      }
    } catch (err) {
      let msg = `Failed to ${mode === "edit" ? "update" : "save"} project`;
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
    handleBrowseFolder,
    handleSave,
  };
}
