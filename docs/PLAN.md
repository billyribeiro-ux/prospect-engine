# ProspectEngine — master plan

This document is the single source of truth for architecture, scope, phases, and execution status. It reflects the intended product: **local-business discovery, website auditing, scoring, CRM pipeline, territory map, reporting, and optional ML**—with a **SvelteKit** workspace shell, **Rust (Axum)** API, **PE7 design tokens** (no Tailwind/Lucide in product UI), and **pnpm** monorepo discipline.

---

## 1. Principles

- **Local-first / self-hostable** where possible; clear env boundaries (`PE_*`).
- **Strict TypeScript** in JS packages; **Rust** `clippy` + `deny` patterns for API.
- **Design system**: `@pe/tokens` (CSS variables, themes, density), `@pe/ui` (Svelte primitives), **Iconify** for icons only where needed.
- **No fake scripts**: package `build`/`check` must invoke real tools (`svelte-check`, `tsc`, `cargo`, etc.).
- **Node version**: **24.14.1** (pinned via `.nvmrc` and `engines`); tooling otherwise tracks current stable majors unless pinned for compatibility.

---

## 2. Technology stack (locked)

| Layer | Choice |
|-------|--------|
| App shell | **Svelte 5** + **SvelteKit 2** (`apps/web`) |
| Bundler | **Vite** (version aligned with SvelteKit peer range) |
| CSS | **PE7** token layers in `@pe/tokens` (no Tailwind) |
| Monorepo | **pnpm** workspaces + **Turbo** |
| Lint/format | **Biome** |
| API | **Rust**, **Axum**, **tokio**, **tower-http** (`apps/api`) |
| Persistence (planned) | **PostgreSQL** + **sqlx** in `crates/db` |
| ML service (planned) | **Python** + **FastAPI** (`services/ml`) |
| Desktop (planned) | **Tauri 2** (`apps/desktop`) — not scaffolded yet |

---

## 3. Repository layout

```
prospect-engine/
├── apps/
│   ├── web/                 # SvelteKit app (marketing + authenticated shell + API proxy)
│   └── api/                 # Axum HTTP API
├── packages/
│   ├── tokens/              # PE7 CSS: primitives, semantic, themes, density, reset
│   ├── types/               # Shared TS domain types (path exports, no barrel)
│   ├── ui/                  # Shared Svelte components (consumed from source)
│   └── gsap/                # GSAP actions + animation registry
├── crates/                  # Rust library crates (shared, db, crawler, scorer, …)
├── services/ml/             # FastAPI service (health + future training/inference)
├── docs/PLAN.md             # This file
└── .env.example             # PE_API_ORIGIN, bind host/port, etc.
```

---

## 4. Web application (`apps/web`)

### 4.1 Routes

- **Marketing**: `routes/+page.svelte` (landing).
- **App shell** (`(app)/`): sidebar + tabs + command palette + split pane; workspace routes:
  - `/discover`, `/audit`, `/pipeline`, `/map`, `/reports`, `/settings`
- **Auth** (`(auth)/`): login, register, forgot-password (UI; backend auth is phased).
- **BFF proxy**: `routes/api/v1/[...path]/+server.ts` → Rust `PE_API_ORIGIN`.

### 4.2 State & cross-cutting

- **Stores** (`lib/stores/`): theme, layout, keyboard, discovery, audit, pipeline, map, websocket, etc. (some stubs).
- **Actions** (`lib/actions/`): keyboard, focus trap, GSAP re-exports under `lib/actions/gsap/*`.
- **Handlers**: e.g. `appShellKeydown` for shortcuts.

### 4.3 Components (target tree)

- **layout**: `AppShell`, `Sidebar`, `TabBar`, `CommandPalette`, `PaneDivider`, `ThemeControls`, …
- **discovery**: `DiscoveryPanel`, `SourceConfig`, `RadiusSelector`
- **audit**: `AuditReport`, `DimensionCard`, `ScoreRing`, `CompetitorBench`
- **crm**: `KanbanBoard`, `PipelineFunnel`, `ActivityTimeline`, `LeadCard`
- **map**: `MapView`, `MarkerLayer`, `HeatmapLayer`, `RouteOverlay`
- **email**: `TemplateEditor`, `SendDialog`, `TrackingDashboard` (routes optional until nav grows)
- **reports**: `ReportPreview`, `WhiteLabelConfig`, `ExportDialog`
- **shared**: thin wrappers over `@pe/ui` (`Badge`, `DataGrid`, `VirtualList`, `Modal`, `Toast`, `Tooltip`)

---

## 5. Backend (`apps/api` + workspace crates)

### 5.1 HTTP API (current)

- **Global**: `GET /health`
- **Versioned**: `GET /api/v1/health` and stubs:
  - `GET /api/v1/auth/session`
  - `GET /api/v1/discovery`, `/audit`, `/pipeline`, `/reports`, `/map`
  - `GET /api/v1/ws` → `501` JSON (WebSocket upgrade not implemented)

### 5.2 Crates (directional)

- **shared**: common types/helpers
- **db**: migrations + sqlx (compile-time queries when wired)
- **crawler**, **scorer**, **discovery**, **queue**: domain services (stubs → real implementations)

---

## 6. ML service (`services/ml`)

- **FastAPI** + **uvicorn**; `GET /health` minimum.
- Training/inference pipelines **out of scope** until Phase 3+.

---

## 7. Phases & status

### Phase 0 — Monorepo & design system — **done**

- [x] pnpm workspace, Turbo, Biome, base `tsconfig`, Node 24.14.1 engines
- [x] `@pe/tokens` themes (midnight, dawn, terminal, oled) + density + semantic layers
- [x] `@pe/types` path exports
- [x] `@pe/ui` primitives + `ambient.d.ts` for `tsc` inputs
- [x] `@pe/gsap` registry + actions (fadeIn, stagger, counter, parallax, …)

### Phase 1 — Web shell & navigation — **done**

- [x] App layout, sidebar, tab bar, command palette, split pane
- [x] Theme/density hydration + cookies (document.cookie; Cookie Store API optional later)
- [x] API proxy route to Rust origin
- [x] Auth route **pages** (forms only until backend)

### Phase 1b — Wire workspace UI to feature components — **done**

- [x] Each workspace route composes its feature components (sample data OK)
- [x] Keep i18n titles via `messages.app.shell.nav.*`

### Phase 2 — API hardening — **partial**

- [x] Axum router + JSON stubs
- [ ] Auth (sessions/JWT) + Argon2
- [ ] sqlx + migrations + repository layer
- [ ] Idempotent jobs + queue consumer

### Phase 3 — Domain features — **stub → product**

- [ ] Discovery runs (sources, radius, scheduling)
- [ ] Audit pipeline (dimensions, scores, competitor bench)
- [ ] CRM (kanban, funnel, timeline)
- [ ] Map (MapLibre, markers, heatmap, routes)
- [ ] Email (templates, send, tracking)
- [ ] Reports (preview, white-label, PDF/export)

### Phase 4 — Desktop — **not started**

- [ ] `apps/desktop` Tauri 2 shell embedding `apps/web` or static build

### Phase 5 — ML — **stub**

- [x] FastAPI health
- [ ] Model training/inference contracts + worker

---

## 8. Commands

| Command | Purpose |
|---------|---------|
| `pnpm install` | Install JS deps |
| `pnpm dev` | Turbo dev (web + any configured apps) |
| `pnpm run check` | `turbo run check` + `cargo check --workspace` |
| `pnpm run build` | Production build (Turbo) |
| `pnpm --filter @pe/web run build` | Web-only production build |
| `cargo check --workspace` | Rust compile check |

---

## 9. Environment

See **`.env.example`**. Typical:

- `PE_API_ORIGIN` — Rust API base URL for the SvelteKit proxy
- `PE_API_BIND_HOST` / `PE_API_BIND_PORT` — Axum listen address

---

## 10. Definition of done (release candidate)

- `pnpm run check` and `pnpm run build` green on Node **24.14.1**
- `cargo test` / `cargo clippy` policy defined for CI
- Core user journeys: discover → audit → pipeline → map → reports (even with fixture data)
- API auth + persistence for at least one vertical slice

---

*Last updated: Phase 1b (workspace pages wired to feature components) and `README.md` added in the same change set as this file.*
