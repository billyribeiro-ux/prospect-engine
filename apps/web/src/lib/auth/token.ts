import { browser } from "$app/environment";

const STORAGE_KEY = "pe_jwt";

export function setToken(token: string): void {
	if (!browser) return;
	globalThis.localStorage.setItem(STORAGE_KEY, token);
}

export function getToken(): string | null {
	if (!browser) return null;
	return globalThis.localStorage.getItem(STORAGE_KEY);
}

export function clearToken(): void {
	if (!browser) return;
	globalThis.localStorage.removeItem(STORAGE_KEY);
}
