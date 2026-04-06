<script lang="ts">
	import { layoutState } from "$lib/stores/layout.svelte";

	interface Props {
		ariaLabel: string;
	}

	let { ariaLabel }: Props = $props();

	let dividerEl = $state<HTMLDivElement | undefined>(undefined);

	function onPointerDown(event: PointerEvent): void {
		const divider = dividerEl;
		if (!divider) {
			return;
		}
		divider.setPointerCapture(event.pointerId);
		const startX = event.clientX;
		const startPercent = layoutState.primaryPaneSplitPercent;
		const row = divider.closest(".app-shell__panes");
		const rowEl = row instanceof HTMLElement ? row : null;
		const width = rowEl?.getBoundingClientRect().width ?? 1;
		const captureTarget = divider;

		function onMove(ev: PointerEvent): void {
			const dx = ev.clientX - startX;
			const deltaPercent = (dx / width) * 100;
			layoutState.setPrimaryPaneSplitPercent(startPercent + deltaPercent);
		}

		function onUp(ev: PointerEvent): void {
			captureTarget.releasePointerCapture(ev.pointerId);
			window.removeEventListener("pointermove", onMove);
			window.removeEventListener("pointerup", onUp);
			window.removeEventListener("pointercancel", onUp);
		}

		window.addEventListener("pointermove", onMove);
		window.addEventListener("pointerup", onUp);
		window.addEventListener("pointercancel", onUp);
	}
</script>

<div
	bind:this={dividerEl}
	class="pane-divider"
	role="separator"
	aria-orientation="vertical"
	aria-label={ariaLabel}
	onpointerdown={onPointerDown}
></div>

<style>
	@layer components {
		.pane-divider {
			inline-size: var(--pe-pane-divider-inline-size);
			flex-shrink: 0;
			cursor: col-resize;
			background: var(--pe-border-default);
			touch-action: none;
			transition: background var(--pe-duration-fast) var(--pe-easing-default);
		}

		.pane-divider:hover {
			background: var(--pe-interactive-primary);
		}

	}
</style>
