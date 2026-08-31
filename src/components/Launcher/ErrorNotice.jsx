import React from "react";
import { AlertIcon } from "./Icons";

export function ErrorNotice({ error }) {
  if (!error) return null;

  return (
    <div
      role="alert"
      aria-live="polite"
      className="flex items-center space-x-2.5 px-4 py-3 mx-2 my-2 rounded-lg bg-rose-950/40 border border-rose-900/60 text-rose-300 text-xs"
    >
      <AlertIcon className="w-4 h-4 text-rose-400 shrink-0" aria-hidden="true" />
      <span className="font-medium truncate">{error}</span>
    </div>
  );
}
