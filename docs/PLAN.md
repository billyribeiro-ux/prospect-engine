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
| API | Axum + tokio + sqlx SQLite (`apps/api`) |
| Auth | Argon2 + JWT; SQLite users table |
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
├── .github/workflows/  # CI (Biome, turbo check, web build, fmt, clippy)
├── docs/PLAN.md
└── .env.example
```

---

## 4. Web (`apps/web`)

- Routes: landing, `(app)` workspace (**discover, audit, pipeline, map, reports, email, settings**), `(auth)` login/register/forgot (forgot UI only), **BFF** `api/v1/[...path]` → Rust.
- Auth UI: login/register **POST JSON** to `/api/v1/auth/*`; JWT stored in `localStorage` (`$lib/auth/token`).
- Feature components wired on workspace pages.

---

## 5. API (`apps/api`)

- `GET /health`, `GET /api/v1/health`
- **Auth** (SQLite `data/pe.db`):
  - `POST /api/v1/auth/register` — body `{ email, password }` (password ≥ 8)
  - `POST /api/v1/auth/login`
  - `GET /api/v1/auth/session` — optional `Authorization: Bearer <jwt>`
- Stubs: discovery, audit, pipeline, reports, map, `GET /api/v1/ws` → 501
- Env: `PE_DATABASE_URL` (default `sqlite:data/pe.db`), `PE_JWT_SECRET` (required in release, ≥ 32 chars)

---

## 6. Phases — status

### Phase 0 — Monorepo & design system — **done**

### Phase 1 — Web shell & navigation — **done**

### Phase 1b — Workspace feature UI — **done**

### Phase 2 — API hardening — **done (MVP slice)**

- [x] Axum router + JSON stubs for domain endpoints
- [x] **Auth**: Argon2 + JWT + SQLite + sqlx migrations
- [ ] Refresh tokens / cookie sessions (optional)
- [ ] PostgreSQL + sqlx compile-time queries in `crates/db` (optional migration path)
- [ ] Idempotent jobs + **queue consumer** wired to API (queue crate has `MemoryQueue`)

### Phase 3 — Domain features — **partial**

- [x] UI surfaces for discovery, audit, CRM, map, reports, **email**
- [ ] **Discovery** engine (crawler, scheduling) — crates stubbed
- [ ] **Audit** scoring pipeline — `scorer` stubbed
- [ ] **CRM** persistence — API + DB models
- [ ] **Map** MapLibre integration
- [ ] **Email** send/track providers
- [ ] **Reports** PDF/export server-side

### Phase 4 — Desktop — **done (scaffold)**

- [x] `apps/desktop` Tauri 2, dev URL + `../../web/build` for production assets
- [x] `pnpm --filter @pe/desktop dev` (Tauri) / `build:tauri` for installers
- [x] Turbo `build` for desktop is a no-op so CI does not require native bundler

### Phase 5 — ML — **stub**

- [x] FastAPI health
- [x] `services/ml/src/worker.py` idle loop (exit via `PE_ML_WORKER_EXIT`)
- [ ] Training/inference contracts

---

## 7. Commands

| Command | Purpose |
|---------|---------|
| `pnpm install` | Install JS deps |
| `pnpm dev` | Turbo dev (web; run API separately) |
| `pnpm run check` | `turbo run check` + `cargo clippy -- -D warnings` |
| `pnpm run build` | Turbo production build |
| `cd apps/api && cargo run` | API + SQLite migrations |
| `cd apps/desktop && pnpm dev` | Tauri + web dev server |

---

## 8. CI

GitHub Actions: Node **24.14.1**, `pnpm install --frozen-lockfile`, Biome, turbo check, turbo build, `cargo fmt --check`, `cargo clippy -D warnings`.

---

## 9. Definition of done (this repo)

- `pnpm run check` green on Node **24.14.1**
- `cargo clippy --workspace -- -D warnings` green
- Core shell routes + auth + API proxy + SQLite auth DB
- Tauri app present; optional native `build:tauri` for installers
- Further product depth (crawler, MapLibre, SMTP, PDF) is **Phase 3+** work, not missing scaffolding.

---

*Last updated: Phase 2 auth, email route, Tauri desktop, queue MemoryQueue, ML worker stub, CI, and clippy in root `check`.*
