import { browser } from "$app/environment";

const STORAGE_KEY = "pe_custom_theme_tokens";

class CustomThemeStore {
	/** CSS variable name → color value (typically `#rrggbb` from `<input type="color">`). */
	overrides = $state<Record<string, string>>({});

	constructor() {
		if (browser) {
			this.loadFromStorage();
		}
	}

	loadFromStorage(): void {
		try {
			const raw = localStorage.getItem(STORAGE_KEY);
			if (!raw) {
				this.overrides = {};
				return;
			}
			const parsed = JSON.parse(raw) as unknown;
			if (parsed !== null && typeof parsed === "object" && !Array.isArray(parsed)) {
				const next: Record<string, string> = {};
				for (const [k, v] of Object.entries(parsed)) {
					if (k.startsWith("--pe-") && typeof v === "string" && v.length > 0) {
						next[k] = v;
					}
				}
				this.overrides = next;
			}
		} catch {
			this.overrides = {};
		}
	}

	persist(): void {
		if (!browser) {
			return;
		}
		if (Object.keys(this.overrides).length === 0) {
			localStorage.removeItem(STORAGE_KEY);
			return;
		}
		localStorage.setItem(STORAGE_KEY, JSON.stringify(this.overrides));
	}

	/** Re-apply all overrides to the document root (call after preset theme hydration). */
	applyAll(): void {
		if (!browser) {
			return;
		}
		for (const [key, value] of Object.entries(this.overrides)) {
			document.documentElement.style.setProperty(key, value);
		}
	}

	setToken(varName: string, value: string): void {
		if (!browser) {
			return;
		}
		this.overrides = { ...this.overrides, [varName]: value };
		document.documentElement.style.setProperty(varName, value);
		this.persist();
	}

	clearToken(varName: string): void {
		if (!browser) {
			return;
		}
		const next = { ...this.overrides };
		delete next[varName];
		this.overrides = next;
		document.documentElement.style.removeProperty(varName);
		this.persist();
	}

	/** Remove every custom color and storage. */
	clearAll(): void {
		if (!browser) {
			return;
		}
		for (const key of Object.keys(this.overrides)) {
			document.documentElement.style.removeProperty(key);
		}
		this.overrides = {};
		localStorage.removeItem(STORAGE_KEY);
	}
}

export const customThemeState = new CustomThemeStore();
