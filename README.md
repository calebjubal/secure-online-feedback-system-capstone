# Secure Online Feedback System

A Rust capstone for student feedback and authorized administrative review, guided by the supplied architecture diagram.

## Status

Development foundation only. The project now includes strict request handling, bounded storage and review pagination, defensive response headers, a fail-closed development review guard, and an accessible feedback form. Submissions are stored in process memory and are lost when the server restarts. The diagram's 500K+ requests is an unverified future target with no defined time window. Rust memory safety does not guarantee zero XSS.

| Layer | Starter | Planned |
| --- | --- | --- |
| Frontend | Leptos WebAssembly + Trunk feedback form | Authentication, optional SSR |
| Backend | Axum + Tokio, strict validation, 16 KiB body limit, defensive headers, fail-closed development review guard | Institutional authorization |
| Data | Process-memory development store | PostgreSQL 17, SQLx, migrations, row-level security, append-only audit log |
| Infrastructure | Local development | Pingora/Envoy, TLS, rate limits, WAF, non-root minimal containers, internal mTLS |

Do not use real student data until access controls, retention policies, and security review are implemented.

## Structure

```text
frontend/       Browser application, organized around pages
backend/        API, organized around features
CONTRIBUTING.md Contribution workflow
```

Extend the feature-oriented structure with widgets, entities, and shared modules when needed. Avoid empty architecture layers.

## Run locally

Install current stable Rust and Cargo. The frontend also needs Trunk and the browser target:

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
cd frontend
trunk serve
```

Open <http://127.0.0.1:8080>. In another terminal:

```sh
cd backend
cargo run
```

The API defaults to `127.0.0.1:3000`; override with `BIND_ADDR`. `GET /api/health` returns `{"status":"ok"}` for process liveness only. Trunk proxies `/api/` to the API without permissive CORS.

`POST /api/feedback` accepts an anonymous category and a 1–2,000-character message, then returns an opaque receipt ID. Unexpected JSON fields are rejected so identifiers cannot be added accidentally. API responses include no-store caching and defensive browser security headers. Submissions live only in process memory and disappear on restart; do not use it for real student feedback.

`GET /api/admin/feedback` is disabled by default. For local development only, set a randomly generated `ADMIN_REVIEW_TOKEN` of at least 32 bytes and pass it in the `x-admin-token` header. Short or blank tokens disable the route. This temporary guard is not institutional authentication or production authorization. See [the API contract](docs/feedback-api.md).

## Verify

Run in both application directories:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Also run `trunk build` in `frontend/` to verify the browser bundle. Commit each application's Cargo.lock for reproducible dependency resolution.

## Roadmap

1. Define feedback fields, anonymity, retention, and student/admin permissions.
2. Add database migrations and authorization/RLS integration tests.
3. Implement validated submissions and bounded development admin review with audit events.
4. Add identity integration, edge protections, deployment hardening, and threat-model actions from [the security model](docs/security-model.md).
5. Define and measure load scenarios before making capacity claims.

See [CONTRIBUTING.md](CONTRIBUTING.md) and [the security model](docs/security-model.md). No license has been selected yet.
