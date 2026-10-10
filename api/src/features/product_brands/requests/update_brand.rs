use crate::errors;
use crate::features::product_brands::commands;
use crate::features::product_brands::models;
use crate::features::product_brands::repositories;
use crate::shared::macros::merge;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, garde::Validate, utoipa::ToSchema)]
pub struct UpdateBrand {
    #[garde(required, length(max = 255))]
    #[schema(required, max_length = 255)]
    pub name: Option<String>,
}

impl UpdateBrand {
    pub async fn validate_into(
        self,
        id: models::ProductBrandId,
        pool: &sqlx::PgPool,
    ) -> Result<commands::UpdateBrand, errors::ValidationError> {
        use garde::Validate as _;
        let garde_result = self.validate();

        // name already exists error
        let mut name_result: Result<(), garde::Report> = Ok(());
        if let Some(name) = &self.name {
            let mut conn = pool.acquire().await?;
            let exists = repositories::exists_by_name_except_id(pool, id, name).await?;
            if exists {
                let mut error = garde::Report::new();
                error.append(
                    garde::Path::new("name"),
                    garde::Error::new(format!("changed brand name={name} is already exists")),
                );

                name_result = Err(error);
            }
        }

        match (garde_result, name_result) {
            (Ok(_), Ok(_)) => Ok(commands::UpdateBrand {
                id,
                name: self.name.expect("name is required"),
            }),
            (result1, result2) => {
                let report = merge!(result1, result2);
                Err(errors::ValidationError::Validation(report))
            }
        }
    }
}
