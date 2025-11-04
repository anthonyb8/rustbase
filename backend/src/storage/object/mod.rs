use crate::config::CONFIG;
use crate::error::Error;
use crate::error::Result;
use object_store::aws::AmazonS3Builder;
use object_store::gcp::GoogleCloudStorageBuilder;
use object_store::local::LocalFileSystem;
use object_store::ObjectStore;
use std::sync::Arc;
use url::Url;

pub fn get_object_store(url: &str) -> Result<Arc<dyn ObjectStore>> {
    println!("{:?}", url);
    let parsed = Url::parse(url)?;
    println!("shit");

    match parsed.scheme() {
        // Local filesystem: file:///path/to/data
        "file" => {
            let path = parsed
                .to_file_path()
                .map_err(|_| Error::from("Invalid file path"))?;
            let store = LocalFileSystem::new_with_prefix(path)?;
            Ok(Arc::new(store))
        }

        // AWS S3 or S3-compatible (including MinIO)
        "s3" => {
            let bucket = parsed
                .host_str()
                .ok_or_else(|| Error::from("Missing bucket"))?;
            println!("{:?}", bucket);

            let endpoint = if let Some(port) = parsed.port() {
                Some(format!("{}:{}", parsed.host_str().unwrap(), port))
            } else {
                None
            };

            let mut builder = AmazonS3Builder::new()
                .with_bucket_name(bucket)
                .with_region("us-east-1"); // or get from env/config

            if let Some(ep) = endpoint {
                builder = builder.with_endpoint(&ep).with_allow_http(true);
            }

            // For demo: credentials from env
            builder = builder
                .with_access_key_id(&CONFIG.aws_access)
                .with_secret_access_key(&CONFIG.aws_secret);

            let store = builder.build()?;
            println!("{:?}", store);
            Ok(Arc::new(store))
        }

        // Google Cloud Storage: gcs://bucket
        "gcs" => {
            let bucket = parsed
                .host_str()
                .ok_or_else(|| Error::from("Missing bucket"))?;
            // Provide service account path from env/config
            let store = GoogleCloudStorageBuilder::new()
                .with_bucket_name(bucket)
                .with_service_account_path(&CONFIG.gcp_path)
                .build()?;
            Ok(Arc::new(store))
        }

        scheme => Err(Error::from(format!("Unsupported scheme: {}", scheme))),
    }
}

#[derive(Debug, Clone)]
pub struct ObjectClient {
    pub client: Arc<dyn ObjectStore>,
}

impl ObjectClient {
    pub async fn new() -> Result<Self> {
        let config = &*CONFIG;

        Ok(ObjectClient {
            client: get_object_store(&config.object_url)?,
        })
    }
}
