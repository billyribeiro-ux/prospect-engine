import type { Action } from "svelte/action";
import { shortcutMatches } from "$lib/utils/keyboard";

export interface KeyboardShortcutBinding {
	readonly pattern: string;
	readonly handler: (event: KeyboardEvent) => void;
}

export const keyboardShortcut: Action<HTMLElement, KeyboardShortcutBinding> = (node, initial) => {
	let binding = initial;

	function onKeydown(event: KeyboardEvent): void {
		if (shortcutMatches(event, binding.pattern)) {
			binding.handler(event);
		}
	}

	node.addEventListener("keydown", onKeydown);

	return {
		update(next: KeyboardShortcutBinding) {
			binding = next;
		},
		destroy() {
			node.removeEventListener("keydown", onKeydown);
		},
	};
};
