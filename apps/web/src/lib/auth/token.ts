import { browser } from "$app/environment";

const STORAGE_ACCESS = "pe_jwt";
const STORAGE_REFRESH = "pe_refresh";
const STORAGE_ACCESS_EXPIRES_MS = "pe_access_expires_at_ms";

export function setToken(token: string): void {
	if (!browser) return;
	globalThis.localStorage.setItem(STORAGE_ACCESS, token);
}

export function clearToken(): void {
	if (!browser) return;
	globalThis.localStorage.removeItem(STORAGE_ACCESS);
	globalThis.localStorage.removeItem(STORAGE_REFRESH);
	globalThis.localStorage.removeItem(STORAGE_ACCESS_EXPIRES_MS);
}

export function getToken(): string | null {
	if (!browser) return null;
	return globalThis.localStorage.getItem(STORAGE_ACCESS);
}

export function getRefreshToken(): string | null {
	if (!browser) return null;
	return globalThis.localStorage.getItem(STORAGE_REFRESH);
}

/** Persists access + refresh tokens and approximate access expiry (for proactive refresh). */
export function setAuthSession(token: string, refreshToken: string, expiresInSecs: number): void {
	if (!browser) return;
	globalThis.localStorage.setItem(STORAGE_ACCESS, token);
	globalThis.localStorage.setItem(STORAGE_REFRESH, refreshToken);
	const expiresAt = Date.now() + Math.max(0, expiresInSecs) * 1000;
	globalThis.localStorage.setItem(STORAGE_ACCESS_EXPIRES_MS, String(expiresAt));
}

/** Returns true when access token is missing or past expiry window (with 30s skew). */
export function shouldRefreshAccessToken(): boolean {
	if (!browser) return false;
	const access = globalThis.localStorage.getItem(STORAGE_ACCESS);
	const refresh = globalThis.localStorage.getItem(STORAGE_REFRESH);
	if (!access || !refresh) return false;
	const raw = globalThis.localStorage.getItem(STORAGE_ACCESS_EXPIRES_MS);
	if (!raw) return false;
	const expiresAt = Number.parseInt(raw, 10);
	if (Number.isNaN(expiresAt)) return true;
	return Date.now() >= expiresAt - 30_000;
}

/**
 * Calls `POST /api/v1/auth/refresh` and updates stored tokens on success.
 * Returns `null` if refresh failed or no refresh token.
 */
export async function refreshAccessToken(): Promise<string | null> {
	if (!browser) return null;
	const refresh = getRefreshToken();
	if (!refresh) return null;
	const res = await fetch("/api/v1/auth/refresh", {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify({ refresh_token: refresh }),
	});
	if (!res.ok) {
		clearToken();
		return null;
	}
	const data = (await res.json()) as {
		token?: string;
		refresh_token?: string;
		expires_in?: number;
	};
	if (!data.token || !data.refresh_token || data.expires_in == null) {
		clearToken();
		return null;
	}
	setAuthSession(data.token, data.refresh_token, data.expires_in);
	return data.token;
}
