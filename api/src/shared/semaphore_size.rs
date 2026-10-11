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
    derive_more::Into,
    derive_more::FromStr,
    into_inner::IntoInner,
    serde::Serialize,
    serde::Deserialize,
)]
#[display("{}", _0)]
#[serde(transparent)]
pub struct SemaphoreSize(#[default(4)] usize);
