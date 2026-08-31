import React, { useRef, useImperativeHandle, forwardRef } from "react";
import { CloseIcon } from "../Launcher/Icons";

export const ArgumentsEditor = forwardRef(function ArgumentsEditor(
  {
    title = "Arguments",
    helper = null,
    argumentsList = [],
    onChange,
    placeholder = "e.g. --new-window",
  },
  ref
) {
  const activeInputRef = useRef({ index: 0, cursor: 0, el: null });
  const inputsRef = useRef([]);

  const handleAdd = () => {
    onChange([...argumentsList, ""]);
  };

  const handleRemove = (index) => {
    onChange(argumentsList.filter((_, i) => i !== index));
  };

  const handleChange = (index, value) => {
    const updated = argumentsList.map((arg, i) => (i === index ? value : arg));
    onChange(updated);
  };

  const handleInputFocus = (index, e) => {
    activeInputRef.current = {
      index,
      cursor: e.target.selectionStart || 0,
      el: e.target,
    };
  };

  const handleInputSelect = (index, e) => {
    activeInputRef.current = {
      index,
      cursor: e.target.selectionStart || 0,
      el: e.target,
    };
  };

  useImperativeHandle(ref, () => ({
    insertVariable(varText) {
      if (argumentsList.length === 0) {
        onChange([varText]);
        return;
      }

      const { index, cursor, el } = activeInputRef.current;
      const targetIdx = index < argumentsList.length ? index : argumentsList.length - 1;
      const currentVal = argumentsList[targetIdx] || "";
      const pos = el ? el.selectionStart : cursor;

      const before = currentVal.substring(0, pos);
      const after = currentVal.substring(pos);
      const newVal = `${before}${varText}${after}`;

      handleChange(targetIdx, newVal);

      // Restore focus and cursor position after insertion
      setTimeout(() => {
        const inputEl = inputsRef.current[targetIdx];
        if (inputEl) {
          inputEl.focus();
          const newPos = pos + varText.length;
          inputEl.setSelectionRange(newPos, newPos);
        }
      }, 0);
    },
  }));

  return (
    <div className="space-y-1.5">
      <div className="flex items-center justify-between">
        <label className="text-xs font-medium text-neutral-300">
          {title} {helper && <span className="text-neutral-500 font-normal">({helper})</span>}
        </label>
        <button
          type="button"
          onClick={handleAdd}
          className="text-xs text-blue-400 hover:text-blue-300 font-medium cursor-pointer transition-colors"
        >
          + Add Argument
        </button>
      </div>

      {argumentsList.length === 0 ? (
        <div className="px-3 py-2 rounded-lg bg-neutral-900/50 border border-neutral-800/80 text-[11px] text-neutral-500 italic">
          No arguments configured. Click "+ Add Argument" to add one.
        </div>
      ) : (
        <div className="space-y-1.5 max-h-[120px] overflow-y-auto pr-1">
          {argumentsList.map((arg, idx) => (
            <div key={idx} className="flex items-center space-x-2">
              <input
                ref={(el) => (inputsRef.current[idx] = el)}
                type="text"
                value={arg}
                placeholder={placeholder}
                onFocus={(e) => handleInputFocus(idx, e)}
                onSelect={(e) => handleInputSelect(idx, e)}
                onChange={(e) => handleChange(idx, e.target.value)}
                className="flex-1 px-3 py-1.5 rounded-lg bg-neutral-900 border border-neutral-800 text-xs text-neutral-100 placeholder-neutral-500 focus:outline-none focus:border-blue-500/70 font-mono"
              />
              <button
                type="button"
                onClick={() => handleRemove(idx)}
                aria-label="Remove argument"
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
});
