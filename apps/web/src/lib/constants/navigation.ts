export interface AppNavItem {
	readonly id: "discover" | "audit" | "pipeline" | "map" | "reports" | "email" | "settings";
	readonly href: string;
	readonly icon: string;
}

export const APP_NAV_ITEMS: readonly AppNavItem[] = [
	{ id: "discover", href: "/discover", icon: "ph:compass" },
	{ id: "audit", href: "/audit", icon: "ph:chart-bar" },
	{ id: "pipeline", href: "/pipeline", icon: "ph:kanban" },
	{ id: "map", href: "/map", icon: "ph:map-trifold" },
	{ id: "reports", href: "/reports", icon: "ph:file-text" },
	{ id: "email", href: "/email", icon: "ph:envelope" },
	{ id: "settings", href: "/settings", icon: "ph:gear-six" },
];
