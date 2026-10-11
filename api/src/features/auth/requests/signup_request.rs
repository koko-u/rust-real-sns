use crate::errors;
use crate::features::auth::commands;
use crate::features::users::repositories as u_repositories;
use crate::shared::macros::merge;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, garde::Validate, utoipa::ToSchema)]
pub struct SignupRequest {
    #[garde(required, length(max = 255))]
    #[schema(required, max_length = 255)]
    pub username: Option<String>,

    #[garde(required, email, length(max = 255))]
    #[schema(required, format = "email", max_length = 255)]
    pub email: Option<String>,

    #[garde(required, length(min = 8))]
    #[schema(required, min_length = 8)]
    pub password: Option<String>,
}

impl SignupRequest {
    pub async fn validate_into(
        self,
        pool: &sqlx::PgPool,
    ) -> Result<commands::SignupCommand, errors::ServerError> {
        use garde::Validate as _;
        let garde_result = self.validate();

        // check duplicate username
        let mut username_result = Ok(());
        if let Some(username) = &self.username {
            let username_exists = u_repositories::exists_by_username(pool, username).await?;
            if username_exists {
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::new("username"),
                    garde::Error::new("username already exists"),
                );
                username_result = Err(report);
            }
        }

        // check duplicate email
        let mut email_result = Ok(());
        if let Some(email) = &self.email {
            let email_exists = u_repositories::exists_by_email(pool, email).await?;
            if email_exists {
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::new("email"),
                    garde::Error::new("email already exists"),
                );
                email_result = Err(report);
            }
        }

        // password strength
        let mut password_result = Ok(());
        if let Some(password) = &self.password {
            let username = self.username.as_ref().map(|s| s.as_str()).unwrap_or("");
            let email = self.email.as_ref().map(|s| s.as_str()).unwrap_or("");
            let entropy = zxcvbn::zxcvbn(password, &[username, email]);
            if entropy.score() < zxcvbn::Score::Three {
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::new("password"),
                    garde::Error::new("password is too weak"),
                );
                password_result = Err(report);
            }
        }

        match (garde_result, username_result, email_result, password_result) {
            (Ok(()), Ok(()), Ok(()), Ok(())) => Ok(commands::SignupCommand {
                username: self.username.expect("username should be required"),
                email: self.email.expect("email should be required"),
                password: self.password.expect("password should be required"),
            }),
            (r1, r2, r3, r4) => {
                let report = merge!(r1, r2, r3, r4);
                Err(errors::ServerError::Validation(report))
            }
        }
    }
}
