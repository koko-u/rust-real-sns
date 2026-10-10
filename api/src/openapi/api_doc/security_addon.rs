use utoipa::openapi::security;

pub struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let Some(components) = openapi.components.as_mut() else {
            return;
        };

        components.add_security_scheme(
            "bearerAuth",
            security::SecurityScheme::Http(
                security::Http::builder()
                    .scheme(security::HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );

        let security_requirement = security::SecurityRequirement::new("bearerAuth", Vec::<String>::new());
        openapi
            .security
            .get_or_insert_default()
            .push(security_requirement);
    }
}
