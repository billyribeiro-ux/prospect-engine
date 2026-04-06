/**
 * Returns true when `event` matches a shortcut pattern like `mod+k` or `mod+shift+p`.
 * `mod` matches Meta or Ctrl (whichever is used for platform shortcuts).
 */
export function shortcutMatches(event: KeyboardEvent, pattern: string): boolean {
	const rawParts = pattern.split("+");
	const parts = rawParts.map((p) => p.trim().toLowerCase()).filter((p) => p.length > 0);
	if (parts.length === 0) {
		return false;
	}
	const wantMod = parts.includes("mod");
	const wantShift = parts.includes("shift");
	const wantAlt = parts.includes("alt");
	const keyToken = parts[parts.length - 1];
	if (!keyToken) {
		return false;
	}
	const mod = event.metaKey || event.ctrlKey;
	if (wantMod !== mod) {
		return false;
	}
	if (wantShift !== event.shiftKey) {
		return false;
	}
	if (wantAlt !== event.altKey) {
		return false;
	}
	return event.key.toLowerCase() === keyToken;
}
