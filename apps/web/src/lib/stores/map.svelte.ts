import type { HeatmapPoint, MapMarker } from "@pe/types/map";

class MapViewState {
	markers = $state<readonly MapMarker[]>([]);
	heatmap = $state<readonly HeatmapPoint[]>([]);
	selectedMarkerId = $state<string | undefined>(undefined);

	setMarkers(next: readonly MapMarker[]): void {
		this.markers = next;
	}

	setHeatmap(next: readonly HeatmapPoint[]): void {
		this.heatmap = next;
	}

	selectMarker(id: string | undefined): void {
		this.selectedMarkerId = id;
	}
}

export const mapState = new MapViewState();
