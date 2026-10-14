use crate::config;
use crate::errors;

#[derive(Debug, Clone)]
pub struct StorageService {
    client: aws_sdk_s3::Client,
}

const BUCKET_NAME: &str = "real-sns-uploads";

impl StorageService {
    pub async fn new(config: &config::Config) -> Self {
        let config = config.s3_config().await;
        let client = aws_sdk_s3::Client::from_conf(config);

        Self { client }
    }

    pub async fn upload(
        &self,
        key: impl Into<String>,
        content_type: impl Into<String>,
        body: impl Into<aws_sdk_s3::primitives::ByteStream>,
    ) -> Result<(), errors::ServerError> {
        let result = self
            .client
            .put_object()
            .bucket(BUCKET_NAME)
            .key(key)
            .content_type(content_type)
            .body(body.into())
            .send()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                let sdk_error: aws_sdk_s3::Error = e.into_service_error().into();
                Err(errors::ServerError::S3(Box::new(sdk_error)))
            }
        }
    }

    pub async fn presigned_url(
        &self,
        key: impl Into<String>,
        expires_in: time::Duration,
    ) -> Result<url::Url, errors::ServerError> {
        let presigning_config =
            aws_sdk_s3::presigning::PresigningConfig::expires_in(expires_in.unsigned_abs())?;
        let result = self
            .client
            .get_object()
            .bucket(BUCKET_NAME)
            .key(key)
            .presigned(presigning_config)
            .await;

        match result {
            Ok(presigned_request) => {
                let url = presigned_request.uri().parse::<url::Url>()?;
                Ok(url)
            }
            Err(e) => {
                let sdk_error: aws_sdk_s3::Error = e.into_service_error().into();
                Err(errors::ServerError::S3(Box::new(sdk_error)))
            }
        }
    }

    pub async fn presigned_url_opt(
        &self,
        key: Option<&str>,
        expires_in: time::Duration,
    ) -> Result<Option<url::Url>, errors::ServerError> {
        match key {
            Some(key) => {
                let url = self.presigned_url(key, expires_in).await?;
                Ok(Some(url))
            }
            None => Ok(None),
        }
    }
}
