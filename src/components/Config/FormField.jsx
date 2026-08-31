import React from "react";

export function FormField({
  label,
  required = false,
  error = null,
  helper = null,
  children,
}) {
  return (
    <div className="space-y-1.5">
      <div className="flex items-center justify-between">
        <label className="text-xs font-medium text-neutral-300">
          {label}
          {required && <span className="text-rose-400 ml-1">*</span>}
        </label>
        {helper && <span className="text-[11px] text-neutral-500">{helper}</span>}
      </div>

      {children}

      {error && (
        <p role="alert" className="text-xs text-rose-400 font-medium">
          {error}
        </p>
      )}
    </div>
  );
}
