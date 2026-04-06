-- Outbound email audit trail (SMTP send attempts and stub mode).
CREATE TABLE IF NOT EXISTS email_events (
	id TEXT PRIMARY KEY NOT NULL,
	recipient TEXT NOT NULL,
	subject TEXT NOT NULL,
	body_preview TEXT NOT NULL DEFAULT '',
	status TEXT NOT NULL,
	detail TEXT,
	created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_email_events_created ON email_events (created_at);
