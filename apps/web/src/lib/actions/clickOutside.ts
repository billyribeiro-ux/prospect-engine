import type { Action } from "svelte/action";

export interface ClickOutsideParams {
	readonly handler: (event: MouseEvent) => void;
}

export const clickOutside: Action<HTMLElement, ClickOutsideParams> = (node, initial) => {
	let params = initial;

	function onDocumentClick(event: MouseEvent): void {
		if (event.target instanceof Node && node.contains(event.target)) {
			return;
		}
		params.handler(event);
	}

	document.addEventListener("click", onDocumentClick, true);

	return {
		update(next: ClickOutsideParams) {
			params = next;
		},
		destroy() {
			document.removeEventListener("click", onDocumentClick, true);
		},
	};
};
