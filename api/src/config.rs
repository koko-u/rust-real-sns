use axum::http;
use secrecy::SecretString;
use tower_http::cors;

use crate::shared;

#[derive(Debug, clap::Parser)]
pub struct Config {
    #[arg(long, env = "SERVER_HOST", default_value = "localhost", value_parser = parse_host)]
    host: url::Host,
    #[arg(long, env = "SERVER_PORT", default_value_t = 3000)]
    port: u16,

    #[arg(long, env = "CORS_ALLOW_ORIGINS", value_delimiter = ',', default_value = "*")]
    allow_origins: Vec<String>,

    #[arg(long, env = "DATABASE_HOST", default_value = "localhost", value_parser = parse_host)]
    database_host: url::Host,
    #[arg(long, env = "DATABASE_PORT", default_value_t = 5432)]
    database_port: u16,
    #[arg(long, env = "DATABASE_USER", default_value = "postgres")]
    database_user: String,
    #[arg(long, env = "DATABASE_PASSWORD", default_value = "password")]
    database_password: SecretString,
    #[arg(long, env = "DATABASE_NAME", default_value = "mydb")]
    database_name: String,
    #[arg(long, env = "DATABASE_SSLMODE", default_value_t = SslMode::Prefer)]
    database_sslmode: SslMode,
    #[arg(long, env = "MAX_CONNECTIONS", default_value = "10")]
    max_connections: shared::MaxConnections,

    #[arg(long, env = "KEYCLOAK_URL")]
    keycloak_url: url::Url,
    #[arg(long, env = "REAL_SNS_REALM")]
    realm: String,
    #[arg(long, env = "REAL_SNS_AUDIENCE")]
    audience: String,

    #[arg(long, env = "KEYCLOAK_ADMIN_CLIENT_ID")]
    admin_client_id: String,
    #[arg(long, env = "KEYCLOAK_ADMIN_CLIENT_SECRET")]
    admin_client_secret: SecretString,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, derive_more::Display, derive_more::FromStr)]
enum SslMode {
    #[display("disable")]
    Disable,
    #[display("allow")]
    Allow,
    #[display("prefer")]
    Prefer,
    #[display("require")]
    Require,
    #[display("verify-ca")]
    VerifyCa,
    #[display("verify-full")]
    VerifyFull,
}

impl Config {
    pub fn new() -> Result<Self, clap::Error> {
        use clap::Parser;
        Self::try_parse()
    }

    pub fn allow_origins(&self) -> Result<cors::AllowOrigin, http::header::InvalidHeaderValue> {
        if self.allow_origins.contains(&"*".to_string()) {
            Ok(cors::Any.into())
        } else {
            let mut origins = Vec::<http::HeaderValue>::new();
            for origin in &self.allow_origins {
                let value = http::HeaderValue::from_str(origin)?;
                origins.push(value);
            }
            Ok(origins.into())
        }
    }

    pub fn addrs(&self) -> impl tokio::net::ToSocketAddrs {
        (self.host.to_string(), self.port)
    }

    pub fn database_url(&self) -> String {
        use secrecy::ExposeSecret as _;
        format!(
            "postgresql://{user}:{password}@{host}:{port}/{dbname}?sslmode={sslmode}",
            user = self.database_user,
            password = self.database_password.expose_secret(),
            host = self.database_host,
            port = self.database_port,
            dbname = self.database_name,
            sslmode = self.database_sslmode
        )
    }

    pub fn max_connections(&self) -> shared::MaxConnections {
        self.max_connections
    }

    pub fn keycloak_url(&self) -> &url::Url {
        &self.keycloak_url
    }

    pub fn realm(&self) -> &str {
        &self.realm
    }

    pub fn audience(&self) -> &str {
        &self.audience
    }

    pub fn admin_client_id(&self) -> &str {
        &self.admin_client_id
    }

    pub fn admin_client_secret(&self) -> &SecretString {
        &self.admin_client_secret
    }

    pub fn admin_client_url(&self) -> url::Url {
        let mut url = self.keycloak_url.clone();
        url.path_segments_mut().expect("Keycloak URL is invalid").extend([
            "realms",
            self.realm.as_str(),
            "protocol",
            "openid-connect",
            "token",
        ]);

        url
    }
}

fn parse_host(s: &str) -> Result<url::Host, url::ParseError> {
    url::Host::parse(s)
}
