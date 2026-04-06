CREATE TABLE IF NOT EXISTS leads (
	id TEXT PRIMARY KEY NOT NULL,
	name TEXT NOT NULL,
	email TEXT,
	status TEXT NOT NULL DEFAULT 'new',
	created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_leads_status ON leads (status);
