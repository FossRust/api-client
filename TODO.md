* KDL I/O (lossless + fast):
* kdl for user-facing edits (preserves comments/format) and just-kdl for fast parse into internal structs. 
* Request model + env vars: Define RequestSpec, Collection, Environment(Profile); implement {{var}} interpolation before send.
* Script hooks (Rhai): Add a tiny API: env_get/set, request_set_header, response_json(), assert_eq(). Run pre before building reqwest::Request, post after response. 
* HTTP hardening: Shared Client, per-request timeout, redirects, proxy, retries; stream large responses.
* Layout: Sidebar (collections/history), tabbed editor, split response (headers/body/preview). Floem has built-ins (views, styling, virtual lists). 
* WASM plugins (flagged as plugins): Host with wasmtime (component model) or extism for simpler host↔plugin wiring; define WIT for pre/post transformers and (later) UI injections. 
* That’s enough to compile and hit an endpoint today, with a clear path to KDL configs, scripts, and WASI plugins next.
