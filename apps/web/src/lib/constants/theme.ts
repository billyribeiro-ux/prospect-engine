import type { DensityMode } from "@pe/types/theme";
import type { ThemeId } from "@pe/types/theme";

export const THEME_COOKIE_NAME = "pe_theme";

export const DENSITY_COOKIE_NAME = "pe_density";

/** Seconds — one Gregorian year (365 days). */
export const PREFERENCE_COOKIE_MAX_AGE_SECONDS = 31_536_000 as const;

export const DEFAULT_THEME_ID: ThemeId = "dawn";

export const DEFAULT_DENSITY_MODE: DensityMode = "comfortable";

const THEME_IDS: readonly ThemeId[] = ["midnight", "dawn", "terminal", "oled"];

const DENSITY_MODES: readonly DensityMode[] = ["compact", "comfortable", "spacious"];

function isThemeId(value: string): value is ThemeId {
	return (THEME_IDS as readonly string[]).includes(value);
}

function isDensityMode(value: string): value is DensityMode {
	return (DENSITY_MODES as readonly string[]).includes(value);
}

export function parseThemeCookie(value: string | undefined): ThemeId {
	if (value !== undefined && isThemeId(value)) {
		return value;
	}
	return DEFAULT_THEME_ID;
}

export function parseDensityCookie(value: string | undefined): DensityMode {
	if (value !== undefined && isDensityMode(value)) {
		return value;
	}
	return DEFAULT_DENSITY_MODE;
}
