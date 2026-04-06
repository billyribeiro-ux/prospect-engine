import { SHORTCUT_COMMAND_PALETTE, SHORTCUT_TOGGLE_SIDEBAR } from "$lib/constants/keyboard";
import { appState } from "$lib/stores/app.svelte";
import { shortcutMatches } from "$lib/utils/keyboard";

export function handleAppShellKeydown(event: KeyboardEvent): void {
	if (appState.commandPaletteOpen) {
		if (event.key === "Escape") {
			event.preventDefault();
			appState.closeCommandPalette();
			return;
		}
		if (shortcutMatches(event, SHORTCUT_COMMAND_PALETTE)) {
			event.preventDefault();
			appState.closeCommandPalette();
			return;
		}
		return;
	}

	if (shortcutMatches(event, SHORTCUT_COMMAND_PALETTE)) {
		event.preventDefault();
		appState.openCommandPalette();
		return;
	}

	if (shortcutMatches(event, SHORTCUT_TOGGLE_SIDEBAR)) {
		event.preventDefault();
		appState.toggleSidebar();
	}
}
