export interface MapMarker {
	readonly id: string;
	readonly latitude: number;
	readonly longitude: number;
	readonly label: string;
	readonly score: number | undefined;
}

export interface HeatmapPoint {
	readonly latitude: number;
	readonly longitude: number;
	readonly weight: number;
}

export interface RouteLeg {
	readonly fromMarkerId: string;
	readonly toMarkerId: string;
	readonly distanceMeters: number;
}
