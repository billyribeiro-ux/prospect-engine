<script lang="ts">
import { onMount } from "svelte";
import HeatmapLayer from "$lib/components/map/HeatmapLayer.svelte";
import MapView from "$lib/components/map/MapView.svelte";
import MarkerLayer from "$lib/components/map/MarkerLayer.svelte";
import RouteOverlay from "$lib/components/map/RouteOverlay.svelte";
import { messages } from "$lib/i18n/messages/en";

let routesOn = $state(false);
let markerCount = $state(0);

onMount(() => {
	void (async () => {
		try {
			const res = await fetch("/api/v1/map");
			const data = (await res.json()) as { markers?: unknown[] };
			markerCount = data.markers?.length ?? 0;
		} catch {
			markerCount = 0;
		}
	})();
});
</script>

<div class="workspace-page">
	<h1 class="workspace-page__title">{messages.app.shell.nav.map}</h1>
	<p class="workspace-page__body">
		MapLibre renders below. Marker count reflects CRM leads that include latitude and longitude from the API.
	</p>

	<MapView title="Territory map">
		{#snippet children({ map })}
			<div class="map-layers">
				<MarkerLayer count={markerCount} />
				<HeatmapLayer map={map} visible={true} />
				<div class="map-layers__row">
					<label class="map-layers__toggle">
						<input type="checkbox" bind:checked={routesOn} />
						<span>Route overlay</span>
					</label>
					<RouteOverlay map={map} visible={routesOn} />
				</div>
			</div>
		{/snippet}
	</MapView>
</div>

<style>
	@layer components {
		.workspace-page__title {
			margin: 0;
			font-size: var(--pe-font-size-2xl);
			font-weight: 600;
			color: var(--pe-text-primary);
		}

		.workspace-page__body {
			margin-block-start: var(--pe-space-3);
			margin-block-end: var(--pe-space-6);
			font-size: var(--pe-density-font-size);
			color: var(--pe-text-secondary);
		}

		.map-layers {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-3);
			margin-block-start: var(--pe-space-3);
		}

		.map-layers__row {
			display: flex;
			flex-wrap: wrap;
			align-items: center;
			gap: var(--pe-space-4);
		}

		.map-layers__toggle {
			display: inline-flex;
			align-items: center;
			gap: var(--pe-space-2);
			font-family: var(--pe-font-family);
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}
	}
</style>
