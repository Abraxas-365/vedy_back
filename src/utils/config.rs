use std::env;

pub struct Config {
    pub database_url: String,
    pub aws_region: String,
    pub s3_bucket: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            aws_region: std::env::var("AWS_REGION").expect("AWS_REGION must be set"),
            s3_bucket: std::env::var("S3_BUCKET").expect("S3_BUCKET must be set"),
        }
    }
}
