import type { Action } from "svelte/action";

const FOCUSABLE_SELECTOR =
	'a[href], button:not([disabled]), textarea, input, select, [tabindex]:not([tabindex="-1"])';

function getFocusable(root: HTMLElement): HTMLElement[] {
	const nodes = root.querySelectorAll<HTMLElement>(FOCUSABLE_SELECTOR);
	return Array.from(nodes).filter((el) => !el.hasAttribute("disabled") && el.tabIndex !== -1);
}

export const focusTrap: Action<HTMLElement, undefined> = (node) => {
	const previouslyFocused = document.activeElement;
	const focusable = getFocusable(node);
	const first = focusable[0];
	first?.focus();

	const onKeydown = (event: KeyboardEvent): void => {
		if (event.key !== "Tab") {
			return;
		}
		const items = getFocusable(node);
		if (items.length === 0) {
			return;
		}
		const firstEl = items[0];
		const lastEl = items[items.length - 1];
		if (!firstEl || !lastEl) {
			return;
		}
		if (event.shiftKey) {
			if (document.activeElement === firstEl) {
				event.preventDefault();
				lastEl.focus();
			}
		} else if (document.activeElement === lastEl) {
			event.preventDefault();
			firstEl.focus();
		}
	};

	node.addEventListener("keydown", onKeydown);

	return {
		destroy() {
			node.removeEventListener("keydown", onKeydown);
			if (previouslyFocused instanceof HTMLElement) {
				previouslyFocused.focus();
			}
		},
	};
};
