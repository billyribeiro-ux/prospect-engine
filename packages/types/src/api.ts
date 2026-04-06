export type ApiErrorCode =
	| "not_found"
	| "unauthorized"
	| "validation"
	| "conflict"
	| "rate_limited"
	| "internal";

export interface ApiErrorBody {
	readonly error: string;
	readonly code: ApiErrorCode;
}

export interface ApiSuccessEnvelope<T> {
	readonly data: T;
}
