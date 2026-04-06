import { DEFAULT_API_TIMEOUT_MS } from "$lib/constants/api";

export type ApiClientError =
	| { readonly kind: "network"; readonly message: string }
	| { readonly kind: "http"; readonly status: number; readonly body: string }
	| { readonly kind: "aborted" };

function isApiClientError(value: unknown): value is ApiClientError {
	return (
		typeof value === "object" &&
		value !== null &&
		"kind" in value &&
		typeof (value as ApiClientError).kind === "string"
	);
}

export async function apiFetchJson<T>(
	path: string,
	init?: RequestInit & { readonly timeoutMs?: number },
): Promise<T> {
	const timeoutMs = init?.timeoutMs ?? DEFAULT_API_TIMEOUT_MS;
	const controller = new AbortController();
	const timeoutId = setTimeout(() => {
		controller.abort();
	}, timeoutMs);

	try {
		const response = await fetch(path, {
			...init,
			signal: controller.signal,
		});
		const text = await response.text();
		if (!response.ok) {
			const err: ApiClientError = {
				kind: "http",
				status: response.status,
				body: text,
			};
			throw err;
		}
		if (text.length === 0) {
			return undefined as T;
		}
		return JSON.parse(text) as T;
	} catch (e: unknown) {
		if (isApiClientError(e)) {
			throw e;
		}
		if (e instanceof DOMException && e.name === "AbortError") {
			const err: ApiClientError = { kind: "aborted" };
			throw err;
		}
		const message = e instanceof Error ? e.message : "Unknown network error";
		const err: ApiClientError = { kind: "network", message };
		throw err;
	} finally {
		clearTimeout(timeoutId);
	}
}
