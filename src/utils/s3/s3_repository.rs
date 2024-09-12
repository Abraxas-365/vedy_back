use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::config::{BehaviorVersion, Region};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::types::ObjectCannedAcl;
use aws_sdk_s3::Client as S3Client;
use std::sync::Arc;
use std::time::Duration;

use crate::error::ApiError;
use crate::utils::Config;

#[derive(Debug, Clone)]
pub struct S3Repository {
    client: Arc<S3Client>,
    bucket: String,
}

impl S3Repository {
    pub async fn new() -> Result<Self, ApiError> {
        let config = Config::from_env();

        let region_provider =
            RegionProviderChain::first_try(Region::new(config.aws_region.clone()))
                .or_default_provider()
                .or_else(Region::new("us-east-1"));

        let s3_config = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .region(region_provider.region().await.ok_or_else(|| {
                ApiError::UnexpectedError("Failed to determine AWS region".to_string())
            })?)
            .build();

        let client = S3Client::from_conf(s3_config);

        Ok(Self {
            client: Arc::new(client),
            bucket: config.s3_bucket,
        })
    }

    pub async fn post_presigned_url(
        &self,
        key: &str,
        duration: Duration,
    ) -> Result<String, ApiError> {
        let presigned_req = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .acl(ObjectCannedAcl::PublicRead)
            .presigned(
                PresigningConfig::expires_in(duration)
                    .map_err(|e| ApiError::UnexpectedError(e.to_string()))?,
            )
            .await
            .map_err(|e| ApiError::UnexpectedError(e.to_string()))?;

        Ok(presigned_req.uri().to_string())
    }
}
