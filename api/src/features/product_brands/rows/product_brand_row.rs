use crate::features::product_brands::models;

#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ProductBrandRow {
    pub id: uuid::Uuid,
    pub name: String,
}

impl From<ProductBrandRow> for models::ProductBrand {
    fn from(value: ProductBrandRow) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
        }
    }
}
