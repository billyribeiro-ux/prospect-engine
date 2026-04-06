# ProspectEngine — master plan

Single source of truth for architecture, scope, and **delivery status**. The product is **local-business discovery, auditing, scoring, CRM, map, reporting, and optional ML**, implemented as a **pnpm** monorepo with **SvelteKit**, **Rust (Axum)**, **PE7 tokens**, and optional **Tauri** + **Python ML**.

---

## 1. Principles

- Local-first / self-hostable; `PE_*` env boundaries.
- Strict TypeScript (JS) and **clippy `-D warnings`** (Rust API + workspace crates).
- Design system: `@pe/tokens`, `@pe/ui`, Iconify where needed (no Tailwind in product UI).
- Real package scripts (`svelte-check`, `tsc`, `cargo`, etc.).
- **Node 24.14.1** (see `.nvmrc` / `engines`).

---

## 2. Stack

| Layer | Choice |
|--------|--------|
| Web | Svelte 5 + SvelteKit 2 (`apps/web`) |
| CSS | PE7 (`@pe/tokens`) |
| Monorepo | pnpm + Turbo |
| Lint/format | Biome |
| API | Axum + tokio + sqlx `AnyPool` (`SQLite` / `PostgreSQL`) (`apps/api`) |
| Auth | Argon2 + JWT + opaque refresh tokens; portable SQL migrations |
| Desktop | Tauri 2 (`apps/desktop`) |
| ML | FastAPI (`services/ml`) + worker stub |

---

## 3. Repository layout

```
prospect-engine/
├── apps/web/           # SvelteKit (marketing, app shell, API proxy)
├── apps/api/           # Axum + migrations + JWT auth
├── apps/desktop/       # Tauri 2 shell → Vite dev / web build
├── packages/{tokens,types,ui,gsap}
├── crates/{shared,db,crawler,scorer,discovery,queue}
├── services/ml/        # FastAPI + worker stub
├── .github/workflows/  # CI (Biome, turbo check, web build, fmt, clippy, tests)
├── docs/{PLAN,SECURITY}.md
└── .env.example
```

---

## 4. Web (`apps/web`)

- Routes: landing, `(app)` workspace (**discover, audit, pipeline, map, reports, email, settings**), `(auth)` login/register/forgot (forgot UI only), **BFF** `api/v1/[...path]` → Rust.
- Auth UI: login/register **POST JSON** to `/api/v1/auth/*`; access + refresh JWT flow in `localStorage` (`$lib/auth/token`).
- Feature components wired on workspace pages.

---

## 5. API (`apps/api`)

- `GET /health`, `GET /api/v1/health`
- **Auth** (`PE_DATABASE_URL` — default SQLite `data/pe.db`; optional Postgres via `docker-compose.yml`):
  - `POST /api/v1/auth/register` — body `{ email, password }` (password ≥ 8); returns `{ token, refresh_token, expires_in, user }`
  - `POST /api/v1/auth/login` — same response shape
  - `POST /api/v1/auth/refresh` — body `{ refresh_token }`
  - `GET /api/v1/auth/session` — optional `Authorization: Bearer <jwt>`
- Domain slices: `GET /api/v1/discovery` (optional `?url=` HTML fetch via `crawler`), `GET/POST /api/v1/leads`, `POST /api/v1/email/send` (stub **202**), `GET /api/v1/reports/export` (minimal PDF), stubs for audit, pipeline, map metadata, `GET /api/v1/ws` → **501** (`websocket_not_implemented`)
- **Jobs queue** (in-memory): `POST /api/v1/jobs` `{ job_id }` → **202**; `GET /api/v1/queue/stats` → `{ depth }` (uses `crates/queue` `MemoryQueue`)
- Env: `PE_DATABASE_URL` (default `sqlite:data/pe.db`), `PE_JWT_SECRET` (required in release, ≥ 32 chars), `PE_CORS_ALLOW_ORIGINS` (comma-separated; **required in release**; debug may omit → permissive CORS with warning)
- Enterprise hardening checklist: **§6 Phase 2b**

---

## 6. Phases — status

### Phase 0 — Monorepo & design system — **done**

### Phase 1 — Web shell & navigation — **done**

### Phase 1b — Workspace feature UI — **done**

### Phase 2 — API hardening — **done (MVP slice)**

- [x] Axum router + JSON stubs for domain endpoints
- [x] **Auth**: Argon2 + JWT + sqlx migrations (`AnyPool`: SQLite + PostgreSQL)
- [x] Refresh tokens (opaque, stored hashed; `POST /api/v1/auth/refresh`); cookie sessions remain optional
- [x] **PostgreSQL path**: `PE_DATABASE_URL=postgres://…` + `docker-compose.yml` Postgres service (optional; `crates/db` compile-time queries still future if desired)
- [x] **Job queue wired to API**: `MemoryQueue` in `AppState` (`apps/api/src/state.rs`); `POST /api/v1/jobs`, `GET /api/v1/queue/stats` (background consumer / idempotency = future work)

### Phase 2b — Enterprise API hardening — **done**

- [x] Structured JSON errors with stable `code` (aligned with `packages/types` `ApiErrorCode`)
- [x] `PE_CORS_ALLOW_ORIGINS`: explicit allowlist in **release**; permissive in **debug** when unset (warn)
- [x] Security response headers: `X-Content-Type-Options`, `X-Frame-Options`, `Referrer-Policy`, `Permissions-Policy`
- [x] Request correlation: `x-request-id` (UUID) + `http_request` trace spans with `request_id`
- [x] HTTP integration tests (`apps/api/tests/`) against full middleware stack (`build_http_app`)
- [x] **`docs/SECURITY.md`** — auth model, secrets, HTTP surface, client token storage, observability

### Phase 3 — Domain features — **partial**

- [x] UI surfaces for discovery, audit, CRM, map, reports, **email**
- [x] **Discovery** crawler fetch (`crates/crawler` + optional `?url=` on `GET /api/v1/discovery`); scheduling / scale-out = future
- [ ] **Audit** scoring pipeline — `scorer` stubbed
- [x] **CRM** persistence — `leads` table + `GET/POST /api/v1/leads` (MVP)
- [x] **Map** MapLibre GL in `MapView.svelte` (demo tile style); full markers/heatmap = future
- [ ] **Email** send/track providers (SMTP stub **202** on `POST /api/v1/email/send`)
- [x] **Reports** PDF — minimal `GET /api/v1/reports/export` (printpdf); richer templates = future

### Phase 4 — Desktop — **done (scaffold)**

- [x] `apps/desktop` Tauri 2, dev URL + `../../web/build` for production assets
- [x] `pnpm --filter @pe/desktop dev` (Tauri) / `build:tauri` for installers
- [x] Turbo `build` for desktop is a no-op so CI does not require native bundler

### Phase 5 — ML — **stub**

- [x] FastAPI health
- [x] `services/ml/src/worker.py` idle loop (exit via `PE_ML_WORKER_EXIT`)
- [x] Training/inference **contracts** (`services/ml/src/contracts.py`, `POST /v1/train/jobs`, `POST /v1/predict` stubs) + `packages/types/ml` TypeScript mirror

---

## 7. Commands

| Command | Purpose |
|---------|---------|
| `pnpm install` | Install JS deps |
| `pnpm dev` | Turbo dev (web; run API separately) |
| `pnpm run check` | `pnpm run lint` + `turbo run check` + `cargo fmt --check` + `cargo clippy --workspace --all-targets -- -D warnings` + `cargo test --workspace` |
| `pnpm run build` | Turbo production build |
| `cd apps/api && cargo run` | API + SQLite migrations |
| `cd apps/desktop && pnpm dev` | Tauri + web dev server |

---

## 8. CI

Single **quality** job: Node **24.14.1**, Rust stable (fmt, clippy), `pnpm install --frozen-lockfile`, then **`pnpm run check`** (Biome + turbo check + `cargo fmt --check` + clippy `--all-targets` + workspace tests), then **`pnpm run build`**.

---

## 9. Definition of done (this repo)

- `pnpm run check` green on Node **24.14.1** (includes Biome, `cargo fmt --check`, clippy, tests)
- Phase **2b** items (§6) satisfied for review builds
- Core shell routes + auth + API proxy + SQLite auth DB
- Tauri app present; optional native `build:tauri` for installers
- Further product depth (crawler, MapLibre, SMTP, PDF) is **Phase 3+** work, not missing scaffolding.

---

*Last updated: Phase 2 refresh tokens + Postgres path + Phase 3 CRM/discovery/PDF/MapLibre stubs + ML contracts; `docker compose` for Postgres; `pnpm run check` + build.*
