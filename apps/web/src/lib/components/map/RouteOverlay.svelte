<script lang="ts">
import type { Map as MaplibreMap } from "maplibre-gl";

interface Props {
	map?: MaplibreMap;
	visible?: boolean;
}

const SOURCE_ID = "pe-route-src";
const LAYER_ID = "pe-route-line";

let { map, visible = false }: Props = $props();

function removeRoute(m: MaplibreMap) {
	if (m.getLayer(LAYER_ID)) m.removeLayer(LAYER_ID);
	if (m.getSource(SOURCE_ID)) m.removeSource(SOURCE_ID);
}

$effect(() => {
	const m = map;
	const on = visible;
	if (!m) return;
	let cancelled = false;

	const run = async () => {
		removeRoute(m);
		if (!on || cancelled) return;
		const mapRes = await fetch("/api/v1/map");
		if (cancelled || !mapRes.ok) return;
		const mapData = (await mapRes.json()) as {
			markers?: { id: string }[];
		};
		const ids = mapData.markers ?? [];
		const from = ids[0];
		const to = ids[1];
		if (!from || !to) return;
		const fromId = from.id;
		const toId = to.id;
		const q = new URLSearchParams({ from_id: fromId, to_id: toId });
		const routeRes = await fetch(`/api/v1/map/route?${q}`);
		if (cancelled || !routeRes.ok) return;
		const body = (await routeRes.json()) as {
			geojson?: { type: string; coordinates: number[][] };
			distance_meters?: number;
		};
		const gj = body.geojson;
		if (!gj || gj.type !== "LineString" || cancelled) return;
		const line = {
			type: "LineString" as const,
			coordinates: gj.coordinates,
		};

		m.addSource(SOURCE_ID, {
			type: "geojson",
			data: {
				type: "Feature",
				properties: { distance_meters: body.distance_meters ?? 0 },
				geometry: line,
			},
		});
		m.addLayer({
			id: LAYER_ID,
			type: "line",
			source: SOURCE_ID,
			paint: {
				"line-color": "#2563eb",
				"line-width": 3,
				"line-opacity": 0.85,
			},
		});

		const coords = line.coordinates.filter(
			(c): c is [number, number] =>
				Array.isArray(c) && c.length >= 2 && typeof c[0] === "number" && typeof c[1] === "number",
		);
		if (coords.length >= 2) {
			const lons = coords.map((c) => c[0]);
			const lats = coords.map((c) => c[1]);
			const minLon = Math.min(...lons);
			const maxLon = Math.max(...lons);
			const minLat = Math.min(...lats);
			const maxLat = Math.max(...lats);
			m.fitBounds(
				[
					[minLon, minLat],
					[maxLon, maxLat],
				],
				{ padding: 48, duration: 600, maxZoom: 12 },
			);
		}
	};

	const onLoad = () => {
		void run();
	};

	if (m.loaded()) void run();
	else m.once("load", onLoad);

	return () => {
		cancelled = true;
		m.off("load", onLoad);
		removeRoute(m);
	};
});
</script>

<div class="pe-route-overlay" class:pe-route-overlay--on={visible} role="presentation">
	<span class="pe-route-overlay__label">Route (first two markers)</span>
</div>

<style>
	@layer components {
		.pe-route-overlay {
			padding: var(--pe-space-2) var(--pe-space-3);
			border-radius: var(--pe-radius-sm);
			border: 1px solid var(--pe-border-subtle);
			font-family: var(--pe-font-family);
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}

		.pe-route-overlay--on {
			border-color: var(--pe-interactive-primary);
			color: var(--pe-text-primary);
		}
	}
</style>
