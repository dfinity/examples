/**
 * Copy text to the clipboard, returning whether it succeeded.
 *
 * Falls back to a legacy textarea + `execCommand("copy")` when the async
 * Clipboard API is unavailable: the asset canister's default Permissions-Policy
 * sets `clipboard-write=()`, which blocks `navigator.clipboard` — the legacy
 * path is not gated by that policy.
 */
export async function copyToClipboard(text: string): Promise<boolean> {
  try {
    if (navigator.clipboard?.writeText) {
      await navigator.clipboard.writeText(text);
      return true;
    }
  } catch {
    // Blocked (e.g. Permissions-Policy) — fall through to the legacy path.
  }

  try {
    const textarea = document.createElement("textarea");
    textarea.value = text;
    textarea.setAttribute("readonly", "");
    textarea.style.position = "fixed";
    textarea.style.left = "-9999px";
    document.body.appendChild(textarea);
    textarea.select();
    const ok = document.execCommand("copy");
    document.body.removeChild(textarea);
    return ok;
  } catch {
    return false;
  }
}
