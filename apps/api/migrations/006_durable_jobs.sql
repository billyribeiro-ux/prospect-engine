-- Durable job queue processed by the API background worker (`services/worker`).
CREATE TABLE IF NOT EXISTS durable_jobs (
	id TEXT PRIMARY KEY NOT NULL,
	kind TEXT NOT NULL,
	payload TEXT NOT NULL,
	status TEXT NOT NULL DEFAULT 'pending',
	result_summary TEXT,
	error TEXT,
	created_at TEXT NOT NULL,
	updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_durable_jobs_status ON durable_jobs (status, created_at);
