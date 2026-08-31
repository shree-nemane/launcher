/**
 * Analyzes the current command input to extract the active token and preceding prefix.
 * @param {string} input - The raw search input.
 * @returns {{ activeToken: string, prefix: string, isSlash: boolean }}
 */
export function getActiveTokenInfo(input) {
  if (!input) {
    return { activeToken: "", prefix: "", isSlash: false };
  }

  // If input ends with a space, user is ready for a new token
  if (input.endsWith(" ")) {
    return { activeToken: "", prefix: input, isSlash: false };
  }

  const lastSpaceIdx = input.lastIndexOf(" ");
  if (lastSpaceIdx === -1) {
    const token = input.trim();
    return { activeToken: token, prefix: "", isSlash: token.startsWith("/") };
  }

  const prefix = input.substring(0, lastSpaceIdx + 1);
  const activeToken = input.substring(lastSpaceIdx + 1);
  return { activeToken, prefix, isSlash: activeToken.startsWith("/") };
}

/**
 * Applies a selected suggestion (command) into the input string without replacing the entire string.
 * @param {string} currentInput - Current raw input.
 * @param {string} suggestionCommand - The command string to insert (e.g. "deepfake", "/v", "//").
 * @returns {string} The new input string with a trailing space.
 */
export function applySuggestionToInput(currentInput, suggestionCommand) {
  if (!currentInput || currentInput.trim() === "") {
    return `${suggestionCommand} `;
  }

  if (currentInput.endsWith(" ")) {
    return `${currentInput}${suggestionCommand} `;
  }

  const lastSpaceIdx = currentInput.lastIndexOf(" ");
  if (lastSpaceIdx === -1) {
    return `${suggestionCommand} `;
  }

  const prefix = currentInput.substring(0, lastSpaceIdx + 1);
  return `${prefix}${suggestionCommand} `;
}
