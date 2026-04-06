-- Optional map coordinates for CRM leads (Phase 3 map markers).
ALTER TABLE leads ADD COLUMN latitude REAL;
ALTER TABLE leads ADD COLUMN longitude REAL;
