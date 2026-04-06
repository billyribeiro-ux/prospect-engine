<script lang="ts">
import type { Snippet } from "svelte";

interface Props {
	title?: string;
	children?: Snippet;
}

let { title = "Map", children }: Props = $props();
</script>

<div class="pe-map-view" role="region" aria-label={title}>
	<div class="pe-map-view__canvas">
		<p class="pe-map-view__placeholder">Map canvas</p>
	</div>
	{#if children}{@render children()}{/if}
</div>

<style>
	@layer components {
		.pe-map-view {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-3);
			font-family: var(--pe-font-family);
			color: var(--pe-text-primary);
		}

		.pe-map-view__canvas {
			min-block-size: 12rem;
			border-radius: var(--pe-radius-md);
			border: 1px dashed var(--pe-border-default);
			display: grid;
			place-items: center;
			background: var(--pe-surface-sunken);
		}

		.pe-map-view__placeholder {
			margin: 0;
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}
	}
</style>
