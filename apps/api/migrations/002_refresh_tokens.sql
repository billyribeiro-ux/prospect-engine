CREATE TABLE IF NOT EXISTS refresh_tokens (
	id TEXT PRIMARY KEY NOT NULL,
	user_id TEXT NOT NULL,
	token_hash TEXT NOT NULL,
	expires_at TEXT NOT NULL,
	created_at TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_refresh_token_hash ON refresh_tokens (token_hash);
CREATE INDEX IF NOT EXISTS idx_refresh_user ON refresh_tokens (user_id);
