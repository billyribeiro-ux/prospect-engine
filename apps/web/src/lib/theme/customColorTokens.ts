import type { CssVarProbe } from "./cssVarColor";

/** i18n keys under `messages.app.settings.customColors.*` */
export interface CustomColorLabels {
	readonly surfaces: string;
	readonly text: string;
	readonly interactive: string;
	readonly status: string;
	readonly borders: string;
	readonly scores: string;
	readonly surfaceBase: string;
	readonly surfaceRaised: string;
	readonly surfaceOverlay: string;
	readonly surfaceSunken: string;
	readonly textPrimary: string;
	readonly textSecondary: string;
	readonly textTertiary: string;
	readonly textInverse: string;
	readonly interactivePrimary: string;
	readonly interactiveHover: string;
	readonly interactiveActive: string;
	readonly statusSuccess: string;
	readonly statusWarning: string;
	readonly statusError: string;
	readonly statusInfo: string;
	readonly borderSubtle: string;
	readonly borderDefault: string;
	readonly borderStrong: string;
	readonly scoreCritical: string;
	readonly scorePoor: string;
	readonly scoreFair: string;
	readonly scoreGood: string;
	readonly scoreExcellent: string;
}

export type CustomColorGroupId =
	| "surfaces"
	| "text"
	| "interactive"
	| "status"
	| "borders"
	| "scores";

export interface CustomColorToken {
	readonly varName: string;
	readonly labelKey: keyof CustomColorLabels;
	readonly probe: CssVarProbe;
}

export interface CustomColorGroup {
	readonly id: CustomColorGroupId;
	readonly tokens: readonly CustomColorToken[];
}

export const CUSTOM_COLOR_GROUPS: readonly CustomColorGroup[] = [
	{
		id: "surfaces",
		tokens: [
			{ varName: "--pe-surface-base", labelKey: "surfaceBase", probe: "surface" },
			{ varName: "--pe-surface-raised", labelKey: "surfaceRaised", probe: "surface" },
			{ varName: "--pe-surface-overlay", labelKey: "surfaceOverlay", probe: "surface" },
			{ varName: "--pe-surface-sunken", labelKey: "surfaceSunken", probe: "surface" },
		],
	},
	{
		id: "text",
		tokens: [
			{ varName: "--pe-text-primary", labelKey: "textPrimary", probe: "foreground" },
			{ varName: "--pe-text-secondary", labelKey: "textSecondary", probe: "foreground" },
			{ varName: "--pe-text-tertiary", labelKey: "textTertiary", probe: "foreground" },
			{ varName: "--pe-text-inverse", labelKey: "textInverse", probe: "foreground" },
		],
	},
	{
		id: "interactive",
		tokens: [
			{
				varName: "--pe-interactive-primary",
				labelKey: "interactivePrimary",
				probe: "foreground",
			},
			{ varName: "--pe-interactive-hover", labelKey: "interactiveHover", probe: "foreground" },
			{
				varName: "--pe-interactive-active",
				labelKey: "interactiveActive",
				probe: "foreground",
			},
		],
	},
	{
		id: "status",
		tokens: [
			{ varName: "--pe-status-success", labelKey: "statusSuccess", probe: "foreground" },
			{ varName: "--pe-status-warning", labelKey: "statusWarning", probe: "foreground" },
			{ varName: "--pe-status-error", labelKey: "statusError", probe: "foreground" },
			{ varName: "--pe-status-info", labelKey: "statusInfo", probe: "foreground" },
		],
	},
	{
		id: "borders",
		tokens: [
			{ varName: "--pe-border-subtle", labelKey: "borderSubtle", probe: "border" },
			{ varName: "--pe-border-default", labelKey: "borderDefault", probe: "border" },
			{ varName: "--pe-border-strong", labelKey: "borderStrong", probe: "border" },
		],
	},
	{
		id: "scores",
		tokens: [
			{ varName: "--pe-score-critical", labelKey: "scoreCritical", probe: "foreground" },
			{ varName: "--pe-score-poor", labelKey: "scorePoor", probe: "foreground" },
			{ varName: "--pe-score-fair", labelKey: "scoreFair", probe: "foreground" },
			{ varName: "--pe-score-good", labelKey: "scoreGood", probe: "foreground" },
			{ varName: "--pe-score-excellent", labelKey: "scoreExcellent", probe: "foreground" },
		],
	},
] as const;
