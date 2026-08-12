# meta-signal-agent agent notes

Read this repo's `ARCHITECTURE.md` before editing. This
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

Edit `ethos/interface.ethos` and the producer-owned authority manifest together.
Regenerate with `META_SIGNAL_AGENT_UPDATE_INTERFACE_ARTIFACTS=1 cargo build`;
never hand-edit `src/schema/lib/generated.rs`. Operational behavior belongs in
`src/schema/lib/behavior.rs` and must use only encoded projected coordinates.

## Protos estate status

Stack: correct-new destination
Status: active component contract, current checkout legacy-wired
This checkout is not proof of correct-new adoption.
