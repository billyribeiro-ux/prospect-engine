export type DiscoverySource =
	| "google_places"
	| "yelp"
	| "bing_places"
	| "csv_import"
	| "maps_scraper";

export interface Business {
	readonly id: string;
	readonly tenantId: string;
	readonly name: string;
	readonly addressLine1: string;
	readonly addressLine2: string | undefined;
	readonly locality: string | undefined;
	readonly region: string | undefined;
	readonly postalCode: string | undefined;
	readonly countryCode: string;
	readonly latitude: number;
	readonly longitude: number;
	readonly phone: string | undefined;
	readonly websiteUrl: string | undefined;
	readonly category: string | undefined;
	readonly source: DiscoverySource;
	readonly createdAt: string;
	readonly updatedAt: string;
}

export interface DiscoveryScanConfig {
	readonly id: string;
	readonly tenantId: string;
	readonly name: string;
	readonly latitude: number;
	readonly longitude: number;
	readonly radiusMeters: number;
	readonly categories: readonly string[];
	readonly sources: readonly DiscoverySource[];
	readonly createdAt: string;
	readonly updatedAt: string;
}
