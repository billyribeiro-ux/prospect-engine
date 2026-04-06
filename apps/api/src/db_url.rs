//! Normalize `PE_DATABASE_URL` for [`sqlx::any::AnyPool`] (`SQLite` and `PostgreSQL`).

/// Returns a URL understood by `AnyPool` (`postgres://...` or `sqlite://...` / `sqlite::memory:`).
#[must_use]
pub fn normalize_database_url(raw: &str) -> String {
    let t = raw.trim();
    if t.is_empty() {
        return "sqlite://data/pe.db".to_string();
    }
    if t.starts_with("postgres://") || t.starts_with("postgresql://") {
        return t.to_string();
    }
    if t.starts_with("sqlite::memory") {
        return t.to_string();
    }
    if t.starts_with("sqlite://") {
        return t.to_string();
    }
    if t.starts_with("sqlite:") {
        let rest = t.trim_start_matches("sqlite:");
        return format!("sqlite://{rest}");
    }
    format!("sqlite://{t}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn postgres_unchanged() {
        assert_eq!(
            normalize_database_url("postgres://u:p@localhost/db"),
            "postgres://u:p@localhost/db"
        );
    }

    #[test]
    fn sqlite_legacy_prefix() {
        assert_eq!(
            normalize_database_url("sqlite:data/pe.db"),
            "sqlite://data/pe.db"
        );
    }

    #[test]
    fn sqlite_memory_passthrough() {
        assert_eq!(
            normalize_database_url("sqlite::memory:?cache=shared"),
            "sqlite::memory:?cache=shared"
        );
    }
}
