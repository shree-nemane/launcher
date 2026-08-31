import React from "react";

const VARIABLES = [
  { label: "{PROJECT_PATH}", desc: "Root folder path" },
  { label: "{PROJECT_URL}", desc: "Configured web URL" },
  { label: "{PROJECT_NAME}", desc: "Project title" },
  { label: "{PROJECT_COMMAND}", desc: "Project command token" },
  { label: "{PROJECT_WORKING_DIRECTORY}", desc: "Custom work directory" },
];

export function VariableChips({ onInsert }) {
  return (
    <div className="space-y-1.5 pt-1">
      <span className="text-[11px] text-neutral-500 font-medium">
        Click variable to insert at cursor:
      </span>
      <div className="flex flex-wrap gap-1.5">
        {VARIABLES.map((v) => (
          <button
            key={v.label}
            type="button"
            title={v.desc}
            onClick={() => onInsert(v.label)}
            className="px-2 py-0.5 rounded bg-neutral-800/80 hover:bg-neutral-700/80 border border-neutral-700 text-neutral-300 hover:text-white font-mono text-[11px] transition-colors cursor-pointer"
          >
            {v.label}
          </button>
        ))}
      </div>
    </div>
  );
}
