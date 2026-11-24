# Contributing

## Before starting

Read [README.md](README.md) for scope and local setup. This repository is a hardened development foundation, not a production deployment. Authentication, persistence, retention, edge controls, and institutional security review remain future work. Discuss changes to anonymity, permissions, retention, or deployment assumptions before implementing them.

## Development setup

1. Install current stable Rust and Cargo.
2. Install `wasm32-unknown-unknown` with `rustup target add wasm32-unknown-unknown`.
3. Install Trunk with `cargo install trunk --locked`.
4. Run `cargo run` inside `backend/`.
5. Run `trunk serve` inside `frontend/` and open <http://127.0.0.1:8080>.

Both applications are independent Cargo packages. Run commands from the appropriate directory. The manifests declare Rust 1.85 as the baseline, with dependency resolution configured to prefer compatible releases. Leptos 0.7 is the initial frontend baseline; evaluate upgrades deliberately.

`backend/.env.example` documents supported variables; it is not loaded automatically. For example, in PowerShell:

```powershell
$env:BIND_ADDR = '127.0.0.1:3000'
$env:RUST_LOG = 'info'
cargo run
```

In a POSIX shell:

```sh
BIND_ADDR=127.0.0.1:3000 RUST_LOG=info cargo run
```

Use synthetic feedback only. Keep credentials and local `.env` files out of Git.

## Making changes

- Use a focused branch, for example `codex/feedback-validation`.
- Keep changes small and describe the user-visible behavior they add.
- Put frontend pages in `frontend/src/pages/` and API features in `backend/src/features/`.
- Add shared modules only when multiple features need them. Keep transport, domain rules, and persistence responsibilities clear.
- Validate input on the server, render user text safely, and never log feedback bodies, tokens, or personal information.
- Do not introduce unsafe Rust without an explicit design review; the current applications forbid it.
- Add meaningful tests for behavior, especially authorization, validation, and failure paths.
- Treat feedback text, credentials, tokens, and request metadata as sensitive. Do not add logging or analytics without a documented privacy review.

## Required checks

Run these in **both** `frontend/` and `backend/`:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Then run `trunk build` from `frontend/`. Check the page in a browser at mobile and desktop widths, with keyboard navigation and visible focus. Start the API and verify `GET /api/health` returns HTTP 200 and `{"status":"ok"}`. Unknown API routes must return 404.

For security-sensitive changes, also test malformed JSON, unexpected fields, non-JSON content types, oversized bodies, missing and invalid admin credentials, pagination bounds, response headers, and that error responses never echo feedback text. Review [the security model](docs/security-model.md) before calling the system production-ready.

Commit the generated `Cargo.lock` for each application once dependency resolution succeeds. The initial scaffolding environment could not reach crates.io, so lockfiles and successful build/test results are pending; formatting was checked. Do not report a blocked check as passing. Record missing tools or network failures and rerun affected checks in a configured environment.

The API body limit protects supported buffered extractors such as `String` and `Json`. Any future streaming/raw-body endpoint needs its own explicit limit and tests. The health endpoint reports liveness only, not database readiness.

## Commits and review

Use descriptive messages such as `feat: validate feedback submissions`, `fix: reject oversized requests`, or `docs: explain local setup`. Keep unrelated work in separate commits.

Pull request descriptions should explain the problem, resulting behavior, validation, and remaining limitations. Include screenshots for visible changes and migration/rollback details for schema changes. Never claim production security or throughput without evidence.

Pushing commits and opening a pull request are separate actions from creating local commits; follow the repository owner's instructions about when to publish work.

## Security reports

Do not include credentials, personal data, or exploit details in public issues. Contact the repository owner privately through an existing trusted channel to agree on a reporting method. A dedicated security contact has not yet been established.
