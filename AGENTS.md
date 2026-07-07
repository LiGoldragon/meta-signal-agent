# meta-signal-agent agent notes

Read this repo's `INTENT.md` and `ARCHITECTURE.md` before editing. This
repository is the owner-only meta wire contract for the `agent` LLM-call
component: provider configuration and lifecycle. Keep daemon behaviour, actors,
storage, process spawning, the provider registry, and text parsing out of this
crate.

`agent` makes OpenAI-compatible provider HTTP API calls; it is NOT an agent
harness (psyche Spirit `iucr`, `f8k7`). A provider is a generic
OpenAI-compatible API (endpoint + model + typed secret-source reference);
adding one is a `ConfigureProvider` message, never a contract change. The
secret source is an Environment, Gopass, or File reference — the secret value
never crosses the wire.

Edit `schema/lib.schema` and regenerate
(`META_SIGNAL_AGENT_UPDATE_SCHEMA_ARTIFACTS=1 cargo build`); never hand-edit
`src/schema/lib.rs`.
