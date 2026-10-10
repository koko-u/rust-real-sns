use crate::features::product_brands::models;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UpdateBrand {
    pub id: models::ProductBrandId,
    pub name: String,
}
