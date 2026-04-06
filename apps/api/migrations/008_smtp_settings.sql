-- Single-row SMTP relay settings (optional; falls back to PE_SMTP_* when disabled or empty).
CREATE TABLE IF NOT EXISTS smtp_settings (
	id INTEGER PRIMARY KEY NOT NULL CHECK (id = 1),
	enabled INTEGER NOT NULL DEFAULT 0,
	host TEXT NOT NULL DEFAULT '',
	port INTEGER NOT NULL DEFAULT 587,
	username TEXT,
	password_encrypted TEXT,
	from_address TEXT NOT NULL DEFAULT '',
	updated_at TEXT NOT NULL
);
