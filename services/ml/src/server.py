"""FastAPI entrypoint for the ProspectEngine ML scoring microservice."""

from fastapi import FastAPI

from .contracts import (
	MlJobStatus,
	MlPredictRequest,
	MlPredictResponse,
	MlTrainJobCreateRequest,
	MlTrainJobCreateResponse,
)

app = FastAPI(title="ProspectEngine ML", version="1.0.0")


@app.get("/health")
def health() -> dict[str, str]:
	return {"status": "ok"}


@app.post(
	"/v1/train/jobs",
	response_model=MlTrainJobCreateResponse,
	response_model_by_alias=True,
)
def create_train_job(body: MlTrainJobCreateRequest) -> MlTrainJobCreateResponse:
	_ = body
	return MlTrainJobCreateResponse(job_id="stub-job", status=MlJobStatus.QUEUED)


@app.post(
	"/v1/predict",
	response_model=MlPredictResponse,
	response_model_by_alias=True,
)
def predict(body: MlPredictRequest) -> MlPredictResponse:
	return MlPredictResponse(
		scores=[0.0 for _ in body.features],
		model_version=body.model_version,
	)
