//! Normalize `PE_DATABASE_URL` for [`sqlx::any::AnyPool`] (`SQLite` and `PostgreSQL`).

use std::path::{Path, PathBuf};

/// Returns a URL understood by `AnyPool` (`postgres://...` or `sqlite://...` / `sqlite::memory:`).
///
/// Legacy `sqlite:relative/path.db` is resolved against the **`api` crate manifest directory**
/// ([`CARGO_MANIFEST_DIR`]) so the file path is never mis-parsed as `sqlite://relative` (host
/// `relative`), and opening the DB works even when the process cwd is not `apps/api`.
#[must_use]
pub fn normalize_database_url(raw: &str) -> String {
    let t = raw.trim();
    if t.is_empty() {
        return resolve_sqlite_file("data/pe.db");
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
        return resolve_sqlite_file(rest);
    }
    resolve_sqlite_file(t)
}

fn api_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Ensures parent directories exist for a normalized `sqlite://...` file URL (not in-memory).
///
/// # Errors
///
/// Returns I/O errors from [`std::fs::create_dir_all`].
pub fn ensure_sqlite_parent_dir(db_url: &str) -> std::io::Result<()> {
    if !db_url.starts_with("sqlite://") || db_url.contains("memory") {
        return Ok(());
    }
    let path = db_url.trim_start_matches("sqlite://");
    let path = path.split('?').next().unwrap_or(path);
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn resolve_sqlite_file(path_str: &str) -> String {
    let path = Path::new(path_str);
    let path: PathBuf = if path.is_absolute() {
        path.to_path_buf()
    } else {
        api_root().join(path)
    };
    // `mode=rwc` avoids open failures when the file is new and the URL is parsed strictly.
    format!("sqlite://{}?mode=rwc", path.display())
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
    fn sqlite_legacy_prefix_resolves_to_absolute() {
        let expected = format!(
            "sqlite://{}?mode=rwc",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("data/pe.db")
                .display()
        );
        assert_eq!(normalize_database_url("sqlite:data/pe.db"), expected);
    }

    #[test]
    fn sqlite_memory_passthrough() {
        assert_eq!(
            normalize_database_url("sqlite::memory:?cache=shared"),
            "sqlite::memory:?cache=shared"
        );
    }
}
