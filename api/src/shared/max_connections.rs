use better_default::Default as BetterDefault;
use into_inner::IntoInner;

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
    derive_more::FromStr,
    into_inner::IntoInner,
    serde::Serialize,
    serde::Deserialize,
)]
#[display("{}", _0)]
#[serde(transparent)]
pub struct MaxConnections(#[default(10)] u32);
