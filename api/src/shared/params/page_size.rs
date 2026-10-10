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
    utoipa::ToSchema,
)]
#[display("{}", _0)]
#[serde(transparent)]
#[schema(value_type = u32)]
pub struct PageSize(#[default(10)] u32);
