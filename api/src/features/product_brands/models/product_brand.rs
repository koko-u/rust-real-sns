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
#[schema(value_type = uuid::Uuid)]
pub struct ProductBrandId(#[default(uuid::Uuid::now_v7())] uuid::Uuid);

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductBrand {
    pub id: ProductBrandId,
    pub name: String,
}
