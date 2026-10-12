use axum::body;
use axum_typed_multipart::FieldData;
use axum_typed_multipart::TryFromMultipart;

use crate::errors;
use crate::features::profiles::commands;
use crate::shared::macros::merge;

#[derive(Debug, TryFromMultipart, garde::Validate, utoipa::ToSchema)]
pub struct UpdateProfileRequest {
    #[garde(required, length(max = 255))]
    pub display_name: Option<String>,

    #[garde(length(max = 4000))]
    pub biography: Option<String>,

    #[garde(url, length(max = 2000))]
    pub website_url: Option<String>,

    #[garde(length(max = 4000))]
    pub location: Option<String>,

    #[garde(skip)]
    #[form_data(limit = "1MB")]
    #[schema(value_type = String, format = Binary)]
    pub avatar: Option<FieldData<body::Bytes>>,
}

impl UpdateProfileRequest {
    pub fn validate_into(self) -> Result<commands::UpdateProfileCommand, errors::ServerError> {
        use garde::Validate as _;
        let garde_result = self.validate();

        let mut avatar_content_type = mime::APPLICATION_OCTET_STREAM;
        let avatar_result = 'check: {
            let Some(avatar) = &self.avatar else {
                break 'check Ok(());
            };

            match &avatar.metadata.content_type {
                Some(content_type) => match content_type.parse::<mime::Mime>() {
                    Ok(content_type) => {
                        if content_type.type_() == mime::IMAGE {
                            avatar_content_type = content_type;
                            break 'check Ok(());
                        }

                        let mut report = garde::Report::new();
                        report.append(
                            garde::Path::new("avatar"),
                            garde::Error::new("avatar data is not a image type"),
                        );
                        Err(report)
                    }
                    Err(_) => {
                        let mut report = garde::Report::new();
                        report.append(
                            garde::Path::new("avatar"),
                            garde::Error::new("avatar data has invalid content type"),
                        );
                        Err(report)
                    }
                },
                None => {
                    let mut report = garde::Report::new();
                    report.append(
                        garde::Path::new("avatar"),
                        garde::Error::new("avatar data has no content type"),
                    );
                    Err(report)
                }
            }
        };

        match (garde_result, avatar_result) {
            (Ok(_), Ok(_)) => Ok(commands::UpdateProfileCommand {
                display_name: self.display_name.expect("display name should be required"),
                biography: self.biography,
                website_url: self
                    .website_url
                    .map(|url| url.parse().expect("website url should be valid form")),
                location: self.location,
                avatar_data: self.avatar.map(|field_data| field_data.contents),
                avatar_content_type,
            }),
            (r1, r2) => {
                let report = merge!(r1, r2);
                Err(errors::ServerError::Validation(report))
            }
        }
    }
}
