import type { Action } from "svelte/action";

export interface VirtualScrollParams {
	readonly itemCount: number;
	readonly estimateSizePx: number;
	readonly overscan?: number;
	readonly onRangeChange: (startIndex: number, endIndex: number) => void;
}

/**
 * Reports visible index range for virtualized lists. Full virtualization is implemented by the host component.
 */
export const virtualScroll: Action<HTMLElement, VirtualScrollParams> = (node, initial) => {
	let params = initial;
	let frame = 0;

	function measure(): void {
		const rect = node.getBoundingClientRect();
		const viewSize = rect.height;
		const overscan = params.overscan ?? 3;
		const startIndex = Math.max(0, Math.floor(node.scrollTop / params.estimateSizePx) - overscan);
		const visibleCount = Math.ceil(viewSize / params.estimateSizePx) + overscan * 2;
		const endIndex = Math.min(params.itemCount - 1, startIndex + visibleCount);
		params.onRangeChange(startIndex, endIndex);
	}

	function onScroll(): void {
		cancelAnimationFrame(frame);
		frame = requestAnimationFrame(() => {
			measure();
		});
	}

	node.addEventListener("scroll", onScroll, { passive: true });
	measure();

	return {
		update(next: VirtualScrollParams) {
			params = next;
			measure();
		},
		destroy() {
			cancelAnimationFrame(frame);
			node.removeEventListener("scroll", onScroll);
		},
	};
};
