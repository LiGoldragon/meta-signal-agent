use protos::WireContractFamily;
use schema_rust::build::{ContractCrateBuild, CrateName, SchemaVersion, UpdateEnvironmentVariable};

fn main() {
    ContractCrateBuild::from_environment(
        CrateName::new("meta-signal-agent"),
        SchemaVersion::new("0.2.1"),
        UpdateEnvironmentVariable::new("META_SIGNAL_AGENT_UPDATE_SCHEMA_ARTIFACTS"),
        WireContractFamily::MetaSignalSpirit,
    )
    .expect_fresh();
}
