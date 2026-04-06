"""FastAPI entrypoint for the ProspectEngine ML scoring microservice."""

from fastapi import FastAPI

app = FastAPI(title="ProspectEngine ML", version="1.0.0")


@app.get("/health")
def health() -> dict[str, str]:
	return {"status": "ok"}
