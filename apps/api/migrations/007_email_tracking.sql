-- Open/click analytics + click log for outbound email.
ALTER TABLE email_events ADD COLUMN tracking_token TEXT;
ALTER TABLE email_events ADD COLUMN opens INTEGER NOT NULL DEFAULT 0;
ALTER TABLE email_events ADD COLUMN clicks INTEGER NOT NULL DEFAULT 0;

CREATE UNIQUE INDEX IF NOT EXISTS idx_email_events_tracking_token ON email_events (tracking_token);

CREATE TABLE IF NOT EXISTS email_click_log (
	id TEXT PRIMARY KEY NOT NULL,
	tracking_token TEXT NOT NULL,
	target_url TEXT NOT NULL,
	created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_email_click_log_token ON email_click_log (tracking_token);
