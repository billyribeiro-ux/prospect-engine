/**
 * Theme and density identifiers map to `data-theme` and `data-density` on the document root.
 */

export type ThemeId = "midnight" | "dawn" | "terminal" | "oled";

export type DensityMode = "compact" | "comfortable" | "spacious";

/**
 * Semantic color roles used by components; values resolve via CSS custom properties per theme.
 */
export type ColorToken =
	| "surface-base"
	| "surface-raised"
	| "surface-overlay"
	| "surface-sunken"
	| "text-primary"
	| "text-secondary"
	| "text-tertiary"
	| "text-inverse"
	| "interactive-primary"
	| "interactive-hover"
	| "interactive-active"
	| "status-success"
	| "status-warning"
	| "status-error"
	| "status-info"
	| "border-subtle"
	| "border-default"
	| "border-strong";

export interface ThemePreference {
	readonly theme: ThemeId;
	readonly density: DensityMode;
	readonly prefersReducedMotion: boolean;
}
