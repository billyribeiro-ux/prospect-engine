"""Shared request/response shapes for training and inference (mirror `packages/types/src/ml.ts`)."""

from __future__ import annotations

from enum import StrEnum
from typing import Any

from pydantic import BaseModel, ConfigDict, Field


class MlJobStatus(StrEnum):
	QUEUED = "queued"
	RUNNING = "running"
	SUCCEEDED = "succeeded"
	FAILED = "failed"
	CANCELLED = "cancelled"


class MlTrainDatasetRef(BaseModel):
	model_config = ConfigDict(populate_by_name=True)

	tenant_id: str = Field(alias="tenantId")
	uri: str
	checksum_sha256: str | None = Field(default=None, alias="checksumSha256")


class MlTrainJobCreateRequest(BaseModel):
	model_config = ConfigDict(populate_by_name=True)

	dataset: MlTrainDatasetRef
	model_version: str = Field(alias="modelVersion")
	hyperparams: dict[str, Any] | None = None


class MlTrainJobCreateResponse(BaseModel):
	model_config = ConfigDict(populate_by_name=True)

	job_id: str = Field(alias="jobId")
	status: MlJobStatus


class MlPredictRequest(BaseModel):
	model_config = ConfigDict(populate_by_name=True)

	tenant_id: str = Field(alias="tenantId")
	features: list[float]
	model_version: str = Field(alias="modelVersion")


class MlPredictResponse(BaseModel):
	model_config = ConfigDict(populate_by_name=True)

	scores: list[float]
	model_version: str = Field(alias="modelVersion")
