use crate::features::product_brands::models;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct ProductBrand {
    pub id: uuid::Uuid,
    pub name: String,
}

impl From<models::ProductBrand> for ProductBrand {
    fn from(value: models::ProductBrand) -> Self {
        Self {
            id: value.id.into_inner(),
            name: value.name,
        }
    }
}
