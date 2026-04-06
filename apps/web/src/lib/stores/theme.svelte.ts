import type { DensityMode, ThemeId } from "@pe/types/theme";
import { browser } from "$app/environment";
import { invalidate } from "$app/navigation";
import {
	DENSITY_COOKIE_NAME,
	PREFERENCE_COOKIE_MAX_AGE_SECONDS,
	THEME_COOKIE_NAME,
} from "$lib/constants/theme";

class ThemeEngine {
	theme = $state<ThemeId>("dawn");
	density = $state<DensityMode>("comfortable");

	hydrateFromServer(theme: ThemeId, density: DensityMode): void {
		this.theme = theme;
		this.density = density;
		if (!browser) {
			return;
		}
		document.documentElement.dataset.theme = theme;
		document.documentElement.dataset.density = density;
	}

	async setTheme(theme: ThemeId): Promise<void> {
		if (this.theme === theme) {
			return;
		}
		this.theme = theme;
		if (!browser) {
			return;
		}
		document.documentElement.dataset.theme = theme;
		// biome-ignore lint/suspicious/noDocumentCookie: SvelteKit theme cookie sync (Cookie Store API not assumed)
		document.cookie = `${THEME_COOKIE_NAME}=${theme}; Path=/; Max-Age=${String(PREFERENCE_COOKIE_MAX_AGE_SECONDS)}; SameSite=Lax`;
		await invalidate("app:theme");
	}

	async setDensity(density: DensityMode): Promise<void> {
		if (this.density === density) {
			return;
		}
		this.density = density;
		if (!browser) {
			return;
		}
		document.documentElement.dataset.density = density;
		// biome-ignore lint/suspicious/noDocumentCookie: SvelteKit density cookie sync (Cookie Store API not assumed)
		document.cookie = `${DENSITY_COOKIE_NAME}=${density}; Path=/; Max-Age=${String(PREFERENCE_COOKIE_MAX_AGE_SECONDS)}; SameSite=Lax`;
		await invalidate("app:theme");
	}
}

export const themeState = new ThemeEngine();
