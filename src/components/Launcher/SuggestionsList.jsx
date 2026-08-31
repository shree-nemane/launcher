import React, { useRef, useEffect } from "react";
import { SuggestionItem } from "./SuggestionItem";

export function SuggestionsList({
  suggestions,
  selectedIndex,
  onSelectSuggestion,
  onHoverSuggestion,
}) {
  const containerRef = useRef(null);

  // Auto scroll into view when selectedIndex changes via keyboard
  useEffect(() => {
    if (containerRef.current) {
      const activeEl = containerRef.current.children[selectedIndex];
      if (activeEl) {
        activeEl.scrollIntoView({ block: "nearest", behavior: "smooth" });
      }
    }
  }, [selectedIndex]);

  if (!suggestions || suggestions.length === 0) {
    return null;
  }

  return (
    <div
      ref={containerRef}
      id="suggestions-list"
      role="listbox"
      aria-label="Command suggestions"
      className="py-2 px-2 max-h-[260px] overflow-y-auto space-y-0.5"
    >
      {suggestions.map((suggestion, index) => (
        <SuggestionItem
          key={suggestion.id || `${suggestion.command}_${index}`}
          suggestion={suggestion}
          isSelected={index === selectedIndex}
          onSelect={onSelectSuggestion}
          onMouseEnter={() => onHoverSuggestion && onHoverSuggestion(index)}
        />
      ))}
    </div>
  );
}
