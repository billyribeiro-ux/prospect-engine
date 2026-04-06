<script lang="ts">
import { onMount } from "svelte";
import HeatmapLayer from "$lib/components/map/HeatmapLayer.svelte";
import MapView from "$lib/components/map/MapView.svelte";
import MarkerLayer from "$lib/components/map/MarkerLayer.svelte";
import RouteOverlay from "$lib/components/map/RouteOverlay.svelte";
import { messages } from "$lib/i18n/messages/en";

let routesOn = $state(false);
let markerCount = $state(0);
let markers = $state<{ id: string; label: string }[]>([]);
let routeFromId = $state("");
let routeToId = $state("");

onMount(() => {
	void (async () => {
		try {
			const res = await fetch("/api/v1/map");
			const data = (await res.json()) as {
				markers?: { id: string; label?: string }[];
			};
			const raw = data.markers ?? [];
			markers = raw.map((m) => ({
				id: m.id,
				label: m.label?.trim() ? m.label : m.id,
			}));
			markerCount = markers.length;
			const a = markers[0];
			const b = markers[1];
			if (markers.length >= 2 && a && b) {
				routeFromId = a.id;
				routeToId = b.id;
			} else {
				routeFromId = "";
				routeToId = "";
			}
		} catch {
			markers = [];
			markerCount = 0;
			routeFromId = "";
			routeToId = "";
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
					<RouteOverlay
						map={map}
						visible={routesOn}
						fromId={routeFromId}
						toId={routeToId}
					/>
				</div>
				{#if routesOn && markers.length >= 2}
					<div class="map-route-pick">
						<label class="map-route-pick__field">
							<span class="map-route-pick__label">From</span>
							<select bind:value={routeFromId} class="map-route-pick__select">
								{#each markers as m (m.id)}
									<option value={m.id}>{m.label}</option>
								{/each}
							</select>
						</label>
						<label class="map-route-pick__field">
							<span class="map-route-pick__label">To</span>
							<select bind:value={routeToId} class="map-route-pick__select">
								{#each markers as m (m.id)}
									<option value={m.id}>{m.label}</option>
								{/each}
							</select>
						</label>
					</div>
				{/if}
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

		.map-route-pick {
			display: flex;
			flex-wrap: wrap;
			align-items: flex-end;
			gap: var(--pe-space-4);
		}

		.map-route-pick__field {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-1);
			font-family: var(--pe-font-family);
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}

		.map-route-pick__label {
			font-weight: 500;
		}

		.map-route-pick__select {
			min-inline-size: 10rem;
			padding: var(--pe-space-2) var(--pe-space-2);
			border-radius: var(--pe-radius-sm);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-default);
			color: var(--pe-text-primary);
			font: inherit;
		}
	}
</style>
