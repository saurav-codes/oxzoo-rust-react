# oxzoo-rust-react

An official ox deploy example: a Rust 2021 API built with Axum 0.8, fronted by a React 18 single-page app built with Vite 5, deployed to a single Ubuntu VPS by the [ox](https://github.com/saurav-codes/vps-ctl) control plane from one `ox.toml` manifest at the repo root. ox runs `cargo build --release` and `npm install` as install hooks, builds the SPA into `dist/`, starts the compiled `./target/release/server` binary as a systemd process bound to `127.0.0.1:9114`, and configures nginx to serve `dist/` statically while proxying only `/api` and `/health` to the Rust process.

## Stack

| Layer | Tool | Role |
|---|---|---|
| Frontend | React 18 + Vite 5 | SPA built to `dist/`, served by nginx |
| API | Rust + Axum 0.8 | `GET /api/greeting` and `GET /health`, binds `127.0.0.1:9114` |
| Backend build | cargo (apt `rustc`/`cargo`) | compiles `./target/release/server`, which the service then runs |
| Package manager | npm | lockfile (`package-lock.json`) and `Cargo.lock` are committed |
| Deploy | ox | `ox.toml` defines processes, frontend, domain |

## Environment flow

One variable, two paths:

**`GREETING_TAG`**

- **Runtime path (API):** `src/main.rs` reads `std::env::var("GREETING_TAG")` at startup and refuses to boot without it (`.expect("GREETING_TAG must be set")`), so a missing value fails loudly at process start. A restart with a new value changes the API line.
- **Build-time path (SPA):** `vite.config.js` sets `envPrefix: ["GREETING_", "VITE_"]`, so any `GREETING_*` variable in the build environment is exposed to `import.meta.env`. `client/src/App.jsx` renders `import.meta.env.GREETING_TAG`, which is baked into the bundle during `npm run build`. No duplicated `VITE_GREETING_TAG` is needed.

**`PORT`** is injected by the platform into the process environment (see `[deploy]` hooks and `environment_file`); `src/main.rs` reads it with `std::env::var("PORT")` and the process command stays `./target/release/server` with no port in it.

**`cargo build --release`** compiles the release binary into `./target/release/server` inside the release directory during the install hooks; the systemd `web` process then just runs that binary. Build hooks run as the unprivileged project user, so nothing is installed globally; the frontend build needs `nodejs` (NodeSource 22 via `[[apt_sources]]`), the backend needs `rustc` and `cargo`, all from apt.

**Set `GREETING_TAG` in the ox Environment editor BEFORE the first deploy.** The SPA value is baked during the deploy build step, so changing it later requires a redeploy; the API value updates as soon as the process restarts. `.env.example` documents the variable with a placeholder; real values live in the ox dashboard, never in git.

## Deploy with ox

1. Add the repo in the ox dashboard: paste the clone URL `https://github.com/saurav-codes/oxzoo-rust-react.git`.
2. In the Environment editor, set `GREETING_TAG` (for example `v1`).
3. Press **Deploy**. ox runs `cargo build --release`, then `npm install`, then `npm run build`, starts `./target/release/server`, and waits for `http://127.0.0.1:9114/health` to return `ok`.

## Expected output

Visiting the domain shows the project heading plus the two labeled lines:

```
oxzoo-rust-react
frontend: hello world oxzoo-rust-react_<GREETING_TAG>
backend: hello world oxzoo-rust-react_<GREETING_TAG>
```

`<GREETING_TAG>` is whatever you set in the Environment editor. `backend:` shows `loading` until the fetch resolves, and an error message if `/api/greeting` fails.

## How nginx fits

ox configures nginx with `spa = true`: it serves `dist/` from the current release with `try_files $uri $uri/ /index.html`, so deep links fall back to the SPA entry. Only the `[frontend].api_paths` prefixes `/api` and `/health` are proxied to the web process on `127.0.0.1:9114`; everything else is static files.

## Local development

```bash
cargo build --release
GREETING_TAG=localtest PORT=9114 ./target/release/server   # API on 127.0.0.1
npm install
GREETING_TAG=localtest npm run build                       # bakes GREETING_TAG into dist/
```

Pass env inline per the commands above; never commit a real `.env`.
