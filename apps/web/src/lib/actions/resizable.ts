import type { Action } from "svelte/action";

export interface ResizableParams {
	readonly onResize: (deltaPixels: number) => void;
	readonly axis: "inline" | "block";
}

export const resizable: Action<HTMLElement, ResizableParams> = (node, initial) => {
	let params = initial;
	let active = false;
	let start = 0;

	function onPointerDown(event: PointerEvent): void {
		active = true;
		start = params.axis === "inline" ? event.clientX : event.clientY;
		node.setPointerCapture(event.pointerId);
	}

	function onPointerMove(event: PointerEvent): void {
		if (!active) {
			return;
		}
		const current = params.axis === "inline" ? event.clientX : event.clientY;
		const delta = current - start;
		start = current;
		params.onResize(delta);
	}

	function onPointerUp(event: PointerEvent): void {
		active = false;
		node.releasePointerCapture(event.pointerId);
	}

	node.addEventListener("pointerdown", onPointerDown);
	node.addEventListener("pointermove", onPointerMove);
	node.addEventListener("pointerup", onPointerUp);
	node.addEventListener("pointercancel", onPointerUp);

	return {
		update(next: ResizableParams) {
			params = next;
		},
		destroy() {
			node.removeEventListener("pointerdown", onPointerDown);
			node.removeEventListener("pointermove", onPointerMove);
			node.removeEventListener("pointerup", onPointerUp);
			node.removeEventListener("pointercancel", onPointerUp);
		},
	};
};
