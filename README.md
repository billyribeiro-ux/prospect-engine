# ProspectEngine

Monorepo for **ProspectEngine**: discovery, auditing, CRM pipeline, map, and reporting—**SvelteKit** frontend, **Rust (Axum)** API, **PE7** design tokens, **pnpm** workspaces.

## Requirements

- **Node.js 24.14.1** (see `.nvmrc` and `package.json` `engines`)
- **pnpm** 10.x (see root `packageManager`)
- **Rust** toolchain (stable) for `apps/api` and `crates/*`
- **Python 3.11+** optional, for `services/ml`

## Quick start

```bash
nvm use   # or install Node 24.14.1
pnpm install
cp .env.example .env   # set PE_API_ORIGIN; optional PE_JWT_SECRET / PE_DATABASE_URL
```

**Terminal 1 — API** (SQLite + JWT auth; creates `apps/api/data/`):

```bash
cargo run -p api
```

(From repo root; relative `PE_DATABASE_URL` values like `sqlite:data/pe.db` resolve under `apps/api`. You can also `cd apps/api && cargo run`.)

**Terminal 2 — Web** (SvelteKit only; avoids starting Tauri with root `pnpm dev`):

```bash
pnpm --filter @pe/web dev
```

Open the SvelteKit app (default Vite port, often `http://127.0.0.1:5173`). Register at `/register`, then use the workspace shell. The SvelteKit server proxies `/api/v1/*` to `PE_API_ORIGIN` (default `http://127.0.0.1:8080` set in `.env` or `.env.example`). The API also exposes **`POST /api/v1/jobs`** and **`GET /api/v1/queue/stats`** (in-memory queue plus durable rows for development).

### API security notes

- **JWT signing**: Set `PE_JWT_SECRET` in production (minimum 32 characters; use a long random value from a secrets manager). The API signs session tokens with HS256; anyone who knows the secret can forge tokens.
- **CORS**: In **release** builds, set `PE_CORS_ALLOW_ORIGINS` to a comma-separated list of allowed web origins (for example your SvelteKit URL). Debug builds may omit it (permissive CORS with a warning).
- **Client storage**: The web app may persist the JWT in the browser (for example `localStorage`). Treat that as sensitive: XSS in the app could exfiltrate tokens. Prefer tight CSP and dependency hygiene in production.
- **SQLite file**: `PE_DATABASE_URL` points at a file on disk; restrict filesystem permissions and backups accordingly.

See **[docs/SECURITY.md](./docs/SECURITY.md)** for the full security model (auth, headers, correlation IDs, and threat notes).

### Desktop (Tauri)

From repo root after web deps are installed:

```bash
cd apps/desktop && pnpm dev
```

Production installers: `pnpm build:tauri` (requires platform tooling). CI uses a no-op `build` for `@pe/desktop` so pipelines do not require Tauri bundling.

## Scripts

| Script | Description |
|--------|-------------|
| `pnpm dev` | Development via Turbo |
| `pnpm run build` | Production build |
| `pnpm run check` | Biome + turbo check + `cargo fmt` / clippy / tests (see `docs/PLAN.md` §7) |
| `pnpm run lint` | Biome check |
| `pnpm run format` | Biome format |

## Docs

- **[docs/PLAN.md](./docs/PLAN.md)** — architecture, phases, and status
- **[docs/SECURITY.md](./docs/SECURITY.md)** — API security, secrets, and operational guidance

## License

Proprietary unless otherwise noted (`LicenseRef-Proprietary` in Rust workspace).
