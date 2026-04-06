class AppState {
	currentView = $state<"discover" | "audit" | "pipeline" | "map" | "reports">("discover");
	sidebarCollapsed = $state(false);
	commandPaletteOpen = $state(false);
	activePaneId = $state<string>("primary");

	readonly showSidebar = $derived(!this.sidebarCollapsed);

	toggleSidebar(): void {
		this.sidebarCollapsed = !this.sidebarCollapsed;
	}

	openCommandPalette(): void {
		this.commandPaletteOpen = true;
	}

	closeCommandPalette(): void {
		this.commandPaletteOpen = false;
	}
}

export const appState = new AppState();
