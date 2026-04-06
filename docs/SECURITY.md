# Security model

This document summarizes how **ProspectEngine** handles authentication, transport, and operational security for the Axum API (`apps/api`). It is intended for security review and production hardening.

## Authentication

- **Passwords** are hashed with **Argon2** (salt per user). Plain-text passwords are never stored.
- **Sessions** use **JWTs** signed with **HS256** for the **access token** (short TTL, default 15 minutes). Only that algorithm is accepted when validating tokens.
- **Refresh tokens** are opaque random strings stored as **SHA-256** hashes; each login/register/refresh issues a new refresh token (rotation).
- **Token lifetime** — access JWT `exp` is short-lived; refresh tokens are long-lived (default 30 days) and revocable per rotation.
- **Login responses** use the same **401 Unauthorized** for unknown users and wrong passwords to avoid account enumeration.

## Secrets and configuration

| Variable | Purpose |
|----------|---------|
| `PE_JWT_SECRET` | HMAC key for JWT signing. **Required in release builds** (minimum 32 characters). Use a cryptographically random value from a secrets manager. |
| `PE_CORS_ALLOW_ORIGINS` | Comma-separated list of allowed browser `Origin` values. **Required in release builds.** Omit only in local debug (permissive CORS with a warning). |
| `PE_DATABASE_URL` | `SQLite` file URL or `PostgreSQL` connection string. Protect credentials and files with OS permissions and backup policies. |
| `PE_SMTP_HOST` | Optional. When set (with `PE_SMTP_FROM`), `POST /api/v1/email/send` can relay mail via SMTP. **Saved SMTP settings** (Settings UI, table `smtp_settings`) take precedence when enabled with a non-empty host; passwords are stored **AES-256-GCM** encrypted at rest (key derived from `PE_JWT_SECRET`). |
| `PE_SMTP_PORT` | SMTP port (default **587**). |
| `PE_SMTP_USER` / `PE_SMTP_PASSWORD` | Optional SMTP AUTH credentials. |
| `PE_SMTP_FROM` | `From` address (e.g. `ProspectEngine <mail@example.com>`). Defaults to `noreply@localhost` if unset. |

Never commit real secrets. Use `.env` only on developer machines; production should inject env vars via your platform.

## HTTP surface

- **Body size** is capped (32 KiB) for JSON endpoints to limit abuse.
- **CORS** is configurable; production must set explicit origins rather than wildcard behavior.
- **Security headers** on responses include `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, `Referrer-Policy`, and a restrictive `Permissions-Policy`.
- **Request correlation**: responses include **`x-request-id`** (UUID). Clients should log it when reporting issues.

## Email and audit trail

- Outbound sends are recorded in **`email_events`** (`status`: `stub`, `sent`, or `failed`; optional `detail` for errors).
- Without active SMTP (neither saved settings nor `PE_SMTP_*`), the API accepts requests (**202**) and persists a **stub** row only (no network relay). `GET`/`PUT /api/v1/settings/smtp` require a **Bearer JWT**; the API never returns the stored SMTP password (only `has_password`).
- **`POST /api/v1/email/send`** can be rate-limited per client IP (forwarded `X-Forwarded-For` or direct) via **`PE_EMAIL_RATE_LIMIT_PER_MIN`** (default **20**; set **`0`** to disable). Returns **429** with `code: rate_limited` when exceeded.
- Optional open/click tracking uses per-send **`tracking_token`** values; pixel and redirect URLs are under **`/api/v1/email/track/open/{token}`** and **`/api/v1/email/track/click/{token}`**. Set **`PE_PUBLIC_API_ORIGIN`** when clients need absolute URLs in JSON responses.
- Audits may use a headless Chromium **`--dump-dom`** pass when **`PE_CHROME_BIN`** points to a browser binary; otherwise HTML is fetched over HTTP only.

## Client-side token storage

The web app stores access and refresh tokens in browser storage (`localStorage`) for SPA convenience. That implies **XSS is a token theft risk**. Mitigations include a strict **Content Security Policy**, dependency review, and avoiding unsafe HTML injection. For higher assurance, consider httpOnly cookies with CSRF protections (future work).

## Observability

- Structured logs use `RUST_LOG` (e.g. `RUST_LOG=api=info,tower_http=info`). HTTP logs include `request_id` when the request ID middleware runs.

## Threat modeling (short)

| Threat | Mitigation |
|--------|------------|
| Token forgery | Strong `PE_JWT_SECRET`; HS256-only validation |
| Credential stuffing | Argon2; generic errors on login; rate limiting can be added at the edge |
| SQL injection | Parameterized queries via SQLx |
| XSS stealing JWT | CSP + hygiene; document risk for reviewers |
| SMTP abuse | Authenticate API routes for production sends; built-in per-IP rate limit on `POST /email/send` plus edge/WAF limits |

This is not an exhaustive list; add network-level controls (TLS termination, WAF, IP allowlists) as required by your deployment.
