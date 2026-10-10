use better_default::Default as BetterDefault;
#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    BetterDefault,
    derive_more::Display,
    derive_more::From,
    derive_more::Into,
    derive_more::FromStr,
    serde::Serialize,
    serde::Deserialize,
)]
#[display("{}", _0)]
#[serde(transparent)]
pub struct MaxConnections(#[default(10)] u32);
