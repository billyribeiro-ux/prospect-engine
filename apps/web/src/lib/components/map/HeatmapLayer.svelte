<script lang="ts">
import type { Map as MaplibreMap } from "maplibre-gl";

interface Props {
	map?: MaplibreMap;
	visible?: boolean;
}

const SOURCE_ID = "pe-heatmap-src";
const LAYER_ID = "pe-heatmap-layer";

let { map, visible = true }: Props = $props();

function removeHeatmap(m: MaplibreMap) {
	if (m.getLayer(LAYER_ID)) m.removeLayer(LAYER_ID);
	if (m.getSource(SOURCE_ID)) m.removeSource(SOURCE_ID);
}

$effect(() => {
	const m = map;
	const vis = visible;
	if (!m) return;
	let cancelled = false;

	const run = async () => {
		removeHeatmap(m);
		if (!vis || cancelled) return;
		const res = await fetch("/api/v1/map/heatmap");
		if (cancelled || !res.ok) return;
		const data = (await res.json()) as {
			points?: { latitude: number; longitude: number; weight: number }[];
		};
		const pts = data.points ?? [];
		if (pts.length === 0 || cancelled) return;

		m.addSource(SOURCE_ID, {
			type: "geojson",
			data: {
				type: "FeatureCollection",
				features: pts.map((p) => ({
					type: "Feature" as const,
					properties: { w: p.weight },
					geometry: {
						type: "Point" as const,
						coordinates: [p.longitude, p.latitude],
					},
				})),
			},
		});
		m.addLayer({
			id: LAYER_ID,
			type: "heatmap",
			source: SOURCE_ID,
			paint: {
				"heatmap-weight": ["interpolate", ["linear"], ["get", "w"], 0, 0, 1, 1],
				"heatmap-intensity": 1,
				"heatmap-color": [
					"interpolate",
					["linear"],
					["heatmap-density"],
					0,
					"rgba(33,102,172,0)",
					0.2,
					"rgb(103,169,207)",
					0.4,
					"rgb(209,229,240)",
					0.6,
					"rgb(253,219,199)",
					0.8,
					"rgb(239,138,98)",
					1,
					"rgb(178,24,43)",
				],
				"heatmap-radius": 18,
				"heatmap-opacity": 0.75,
			},
		});
	};

	const onLoad = () => {
		void run();
	};

	if (m.loaded()) void run();
	else m.once("load", onLoad);

	return () => {
		cancelled = true;
		m.off("load", onLoad);
		removeHeatmap(m);
	};
});
</script>

<div class="pe-heatmap-layer" class:pe-heatmap-layer--hidden={!visible} role="presentation">
	<span class="pe-heatmap-layer__label">Heatmap (API)</span>
</div>

<style>
	@layer components {
		.pe-heatmap-layer {
			padding: var(--pe-space-2) var(--pe-space-3);
			border-radius: var(--pe-radius-sm);
			border: 1px solid var(--pe-border-subtle);
			font-family: var(--pe-font-family);
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}

		.pe-heatmap-layer--hidden {
			opacity: 0.4;
		}
	}
</style>
