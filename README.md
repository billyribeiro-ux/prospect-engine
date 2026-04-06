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
cd apps/api && cargo run
```

**Terminal 2 — Web**:

```bash
pnpm dev
```

Open the SvelteKit app (default Vite port, often `5173`). Register at `/register`, then use the workspace shell. The SvelteKit server proxies `/api/v1/*` to `PE_API_ORIGIN` (default `http://127.0.0.1:8080`).

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
| `pnpm run check` | Typecheck/lint tasks + `cargo check --workspace` |
| `pnpm run lint` | Biome check |
| `pnpm run format` | Biome format |

## Docs

- **[docs/PLAN.md](./docs/PLAN.md)** — architecture, phases, and status

## License

Proprietary unless otherwise noted (`LicenseRef-Proprietary` in Rust workspace).
