use crate::errors;
use crate::features::auth::commands;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, garde::Validate, utoipa::ToSchema)]
pub struct LoginRequest {
    #[garde(required)]
    pub username: Option<String>,
    #[garde(required)]
    pub password: Option<String>,
}

impl LoginRequest {
    pub fn validate_into(self) -> Result<commands::LoginCommand, errors::ServerError> {
        use garde::Validate as _;
        self.validate()?;
        Ok(commands::LoginCommand {
            username: self.username.expect("username should be required"),
            password: self.password.expect("password should be required"),
        })
    }
}
