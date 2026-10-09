# oxzoo-rust-react

Deployed with [ox](https://deploywithox.com): deploy a repo to your own server with one command, no Docker. [Docs](https://deploywithox.com/docs) · [Guide for this stack](https://deploywithox.com/docs/guides/rust)

An [ox](https://deploywithox.com) deploy example: a Rust API built with Axum 0.8, with a React 18 SPA built by Vite 5, deployed to your own Ubuntu server. ox compiles the release binary, builds the SPA, starts the binary under systemd, and Caddy serves `dist/` while sending only `/api` and `/health` to the Rust process.

## Stack

| Layer | Tool | Role |
|---|---|---|
| Frontend | React 18 + Vite 5 | SPA built to `dist/` |
| API | Rust + Axum 0.8 | `GET /api/greeting` and `GET /health` |
| Toolchains | rust stable, node 24 | installed by ox from `[tools]` |

## ox.toml

```toml
# Rust API (cargo) + React SPA (npm) in one repo.

[app]
start  = "./target/release/server"
health = "/health"

[static]
dir = "dist"
spa = true
api = ["/api", "/health"]

[build]
commands = ["cargo build --release --locked", "npm run build"]

[tools]
rust = "stable"
node = "24"
```

The repo has two languages, so `[build] commands` names both builds. The release build takes a few minutes on a small server.

## Environment flow

- **Run time (API):** `src/main.rs` reads `GREETING_TAG` at startup and refuses to boot without it. It also reads `PORT`, which ox provides.
- **Build time (SPA):** `vite.config.js` sets `envPrefix: ["GREETING_", "VITE_"]`, so `client/src/App.jsx` gets `import.meta.env.GREETING_TAG`, baked into the bundle by `npm run build`. ox sets your variables before the build, and changing one with `ox vars set` redeploys, which rebuilds the SPA.

## Deploy with ox

```sh
curl -fsSL https://deploywithox.com/install.sh | sh
ox login
ox new https://github.com/saurav-codes/oxzoo-rust-react
printf 'GREETING_TAG=demo\n' | ox review oxzoo-rust-react --from-file - --wait
```

The plan, offline:

```console
$ ox check .
ox check . (manifest: ox.toml)

  app.start                  ./target/release/server                              declared
  app.health                 /health                                              declared
  static.dir                 dist                                                 declared
  static.spa                 true                                                 declared
  static.api                 /api, /health                                        declared
  build.install              npm ci                                               detected:package-lock.json
  build.commands[0]          cargo build --release --locked                       declared
  build.commands[1]          npm run build                                        declared
  tools.node                 24                                                   declared
  tools.rust                 stable                                               declared

  Provided by ox: PORT, HOST, OX_ENV, OX_PROJECT, OX_RELEASE, OX_DATA_DIR, PUBLIC_URL, PUBLIC_HOST
  Set on the dashboard before the first deploy: GREETING_TAG

Ready to deploy.
```

## Expected output

```
oxzoo-rust-react
frontend: hello world oxzoo-rust-react_<GREETING_TAG>
backend: hello world oxzoo-rust-react_<GREETING_TAG>
```

The `frontend:` line is baked into the SPA; the `backend:` line comes from `GET /api/greeting`.

## Local development

```sh
npm install && GREETING_TAG=dev npm run build
cargo build --release
GREETING_TAG=dev PORT=9114 ./target/release/server
```
