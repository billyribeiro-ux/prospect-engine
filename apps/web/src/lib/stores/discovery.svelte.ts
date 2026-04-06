import type { DiscoveryScanConfig } from "@pe/types/business";

class DiscoveryState {
	activeScanId = $state<string | undefined>(undefined);
	savedConfigs = $state<readonly DiscoveryScanConfig[]>([]);
	scanInProgress = $state(false);

	setActiveScan(id: string | undefined): void {
		this.activeScanId = id;
	}

	setSavedConfigs(configs: readonly DiscoveryScanConfig[]): void {
		this.savedConfigs = configs;
	}

	setScanInProgress(value: boolean): void {
		this.scanInProgress = value;
	}
}

export const discoveryState = new DiscoveryState();
