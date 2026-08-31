import React from "react";
import { CloseIcon } from "../Launcher/Icons";

export function RunCommandsEditor({ commands = [], onChange }) {
  const handleAdd = () => {
    onChange([...commands, { name: "", command: "" }]);
  };

  const handleRemove = (index) => {
    onChange(commands.filter((_, i) => i !== index));
  };

  const handleChange = (index, field, value) => {
    const updated = commands.map((item, i) => {
      if (i === index) {
        return { ...item, [field]: value };
      }
      return item;
    });
    onChange(updated);
  };

  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between">
        <label className="text-xs font-medium text-neutral-300">
          Run Commands <span className="text-neutral-500 font-normal">(Optional)</span>
        </label>
        <button
          type="button"
          onClick={handleAdd}
          className="text-xs text-blue-400 hover:text-blue-300 font-medium cursor-pointer transition-colors"
        >
          + Add Command
        </button>
      </div>

      {commands.length > 0 && (
        <div className="space-y-2 max-h-[140px] overflow-y-auto pr-1">
          {commands.map((cmd, index) => (
            <div key={index} className="flex items-center space-x-2">
              <input
                type="text"
                placeholder="Label (e.g. Frontend)"
                value={cmd.name}
                onChange={(e) => handleChange(index, "name", e.target.value)}
                className="w-1/3 px-3 py-1.5 rounded-lg bg-neutral-900 border border-neutral-800 text-xs text-neutral-100 placeholder-neutral-500 focus:outline-none focus:border-blue-500/70"
              />
              <input
                type="text"
                placeholder="Command (e.g. npm run dev)"
                value={cmd.command}
                onChange={(e) => handleChange(index, "command", e.target.value)}
                className="flex-1 px-3 py-1.5 rounded-lg bg-neutral-900 border border-neutral-800 text-xs text-neutral-100 placeholder-neutral-500 focus:outline-none focus:border-blue-500/70 font-mono"
              />
              <button
                type="button"
                onClick={() => handleRemove(index)}
                aria-label="Remove command"
                className="p-1.5 rounded-md text-neutral-500 hover:text-rose-400 hover:bg-neutral-800/60 transition-colors cursor-pointer"
              >
                <CloseIcon className="w-3.5 h-3.5" />
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
