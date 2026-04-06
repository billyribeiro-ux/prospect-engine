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
cp .env.example .env   # if present; configure PE_API_ORIGIN for API proxy
pnpm dev
```

Open the SvelteKit app (default Vite port, often `5173`). Run the API from `apps/api` when testing the proxy:

```bash
cd apps/api && cargo run
```

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
