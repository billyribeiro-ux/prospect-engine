# Security model

This document summarizes how **ProspectEngine** handles authentication, transport, and operational security for the Axum API (`apps/api`). It is intended for security review and production hardening.

## Authentication

- **Passwords** are hashed with **Argon2** (salt per user). Plain-text passwords are never stored.
- **Sessions** use **JWTs** signed with **HS256**. Only that algorithm is accepted when validating tokens.
- **Token lifetime** is seven days (`exp` claim). Validation uses a small **clock skew leeway** (60 seconds) for distributed clocks.
- **Login responses** use the same **401 Unauthorized** for unknown users and wrong passwords to avoid account enumeration.

## Secrets and configuration

| Variable | Purpose |
|----------|---------|
| `PE_JWT_SECRET` | HMAC key for JWT signing. **Required in release builds** (minimum 32 characters). Use a cryptographically random value from a secrets manager. |
| `PE_CORS_ALLOW_ORIGINS` | Comma-separated list of allowed browser `Origin` values. **Required in release builds.** Omit only in local debug (permissive CORS with a warning). |
| `PE_DATABASE_URL` | SQLite file URL. Protect the file with OS permissions and backup policies. |

Never commit real secrets. Use `.env` only on developer machines; production should inject env vars via your platform.

## HTTP surface

- **Body size** is capped (32 KiB) for JSON endpoints to limit abuse.
- **CORS** is configurable; production must set explicit origins rather than wildcard behavior.
- **Security headers** on responses include `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, `Referrer-Policy`, and a restrictive `Permissions-Policy`.
- **Request correlation**: responses include **`x-request-id`** (UUID). Clients should log it when reporting issues.

## Client-side token storage

The web app may store JWTs in browser storage (for example `localStorage`) for SPA convenience. That implies **XSS is a token theft risk**. Mitigations include a strict **Content Security Policy**, dependency review, and avoiding `dangerouslySetInnerHTML` patterns. For higher assurance, consider httpOnly cookies with CSRF protections (future work).

## Observability

- Structured logs use `RUST_LOG` (e.g. `RUST_LOG=api=info,tower_http=info`). HTTP logs include `request_id` when the request ID middleware runs.

## Threat modeling (short)

| Threat | Mitigation |
|--------|------------|
| Token forgery | Strong `PE_JWT_SECRET`; HS256-only validation |
| Credential stuffing | Argon2; generic errors on login; rate limiting can be added at the edge |
| SQL injection | Parameterized queries via SQLx |
| XSS stealing JWT | CSP + hygiene; document risk for reviewers |

This is not an exhaustive list; add network-level controls (TLS termination, WAF, IP allowlists) as required by your deployment.
