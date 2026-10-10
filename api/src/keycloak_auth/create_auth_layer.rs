use axum_keycloak_auth::instance;
use axum_keycloak_auth::layer;
use axum_keycloak_auth::role;

use crate::config;

pub fn create_auth_layer<R>(config: &config::Config) -> layer::KeycloakAuthLayer<R>
where
    R: role::Role,
{
    let instance = instance::KeycloakAuthInstance::new(
        instance::KeycloakConfig::builder()
            .server(config.keycloak_url().clone())
            .realm(config.realm().to_string())
            .build(),
    );

    layer::KeycloakAuthLayer::builder()
        .instance(instance)
        .passthrough_mode(axum_keycloak_auth::PassthroughMode::Block)
        .persist_raw_claims(false)
        .expected_audiences(vec![config.audience().to_string()])
        .build()
}
