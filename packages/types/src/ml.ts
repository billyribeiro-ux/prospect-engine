/**
 * Contracts for the FastAPI ML service (`services/ml`).
 * Aligned with Pydantic models in `services/ml/src/contracts.py`.
 */

export type MlJobStatus = "queued" | "running" | "succeeded" | "failed" | "cancelled";

export interface MlTrainDatasetRef {
	readonly tenantId: string;
	readonly uri: string;
	readonly checksumSha256?: string;
}

export interface MlTrainJobCreateRequest {
	readonly dataset: MlTrainDatasetRef;
	readonly modelVersion: string;
	readonly hyperparams?: Readonly<Record<string, number | string | boolean>>;
}

export interface MlTrainJobCreateResponse {
	readonly jobId: string;
	readonly status: MlJobStatus;
}

export interface MlPredictRequest {
	readonly tenantId: string;
	readonly features: readonly number[];
	readonly modelVersion: string;
}

export interface MlPredictResponse {
	readonly scores: readonly number[];
	readonly modelVersion: string;
}
