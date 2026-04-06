export const messages = {
	app: {
		skipToContent: "Skip to main content",
		productName: "ProspectEngine",
		shell: {
			commandPalette: {
				title: "Command palette",
				searchPlaceholder: "Search navigation and actions",
				empty: "No matches",
				close: "Close",
			},
			sidebar: {
				navLabel: "Primary navigation",
				toggle: "Toggle sidebar",
			},
			tabBar: {
				label: "Workspace areas",
			},
			toolbar: {
				toggleSplit: "Toggle secondary pane",
			},
			primaryPane: {
				ariaLabel: "Main workspace",
			},
			secondaryPane: {
				title: "Secondary pane",
				placeholder: "Secondary content and tools will appear here.",
			},
			nav: {
				discover: "Discover",
				audit: "Audit",
				pipeline: "Pipeline",
				map: "Map",
				reports: "Reports",
				settings: "Settings",
			},
		},
		theme: {
			sectionLabel: "Theme",
			midnight: "Midnight",
			dawn: "Dawn",
			terminal: "Terminal",
			oled: "OLED",
		},
		density: {
			sectionLabel: "Density",
			compact: "Compact",
			comfortable: "Comfortable",
			spacious: "Spacious",
		},
		home: {
			title: "ProspectEngine",
			tagline: "Local business discovery, auditing, and lead management.",
		},
	},
} as const;

export type ShellNavLabels = (typeof messages)["app"]["shell"]["nav"];
