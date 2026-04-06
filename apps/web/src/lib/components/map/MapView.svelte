<script lang="ts">
import type { Map as MaplibreMap } from "maplibre-gl";
import type { Snippet } from "svelte";
import { onMount } from "svelte";
import { browser } from "$app/environment";
import "maplibre-gl/dist/maplibre-gl.css";

interface Props {
	title?: string;
	children?: Snippet<[{ map: MaplibreMap | undefined }]>;
}

let { title = "Map", children }: Props = $props();

let mapEl: HTMLDivElement | undefined;
let map = $state<MaplibreMap | undefined>();

onMount(() => {
	if (!browser || !mapEl) return;
	let cancelled = false;
	void import("maplibre-gl").then((m) => {
		if (cancelled || !mapEl) return;
		map = new m.default.Map({
			container: mapEl,
			style: "https://demotiles.maplibre.org/style.json",
			center: [-98.5, 39.5],
			zoom: 3,
		});
	});
	return () => {
		cancelled = true;
		map?.remove();
		map = undefined;
	};
});
</script>

<div class="pe-map-view" role="region" aria-label={title}>
	<div class="pe-map-view__canvas" bind:this={mapEl}></div>
	{#if children}{@render children({ map })}{/if}
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
			min-block-size: 16rem;
			block-size: min(60vh, 28rem);
			inline-size: 100%;
			border-radius: var(--pe-radius-md);
			border: 1px solid var(--pe-border-default);
			overflow: hidden;
			background: var(--pe-surface-sunken);
		}

		:global(.pe-map-view__canvas .maplibregl-canvas) {
			outline: none;
		}
	}
</style>
