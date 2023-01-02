
use s3::{Bucket, Region};
use s3::creds::Credentials;
use anyhow::Result;

use crate::settings;

pub struct Space {

    bucket: Bucket

}

lazy_static! {

    static ref FULL_PATH: String = format!("https://{}.{}", settings::get("bucket_prefix"), settings::get("storage_endpoint"));

}


impl Space {

    pub fn new() -> Space {

        let bucket_name = settings::get("bucket_prefix");
        let region      = Region::Custom { region: "us-east-1".to_string(), endpoint: settings::get("storage_endpoint") };
        let credentials = Credentials::new(Some(&settings::get("storage_key")), Some(&settings::get("storage_secret")), None, None, None).unwrap();
        let mut bucket  = Bucket::new(&bucket_name, region, credentials).unwrap();

        bucket.add_header("x-amz-acl", "public-read");
        Space {

            bucket: bucket

        }

    }

    pub async fn list_bucket_contents(&self) -> Result<(), anyhow::Error> {

        let results = self.bucket.list("/".to_string(), Some("/".to_string())).await?;

        println!("{:?}", results);
        Ok(())

    }

    pub async fn get_object(&self, object_path: &str) -> Result<Vec<u8>, anyhow::Error> {

        let response_data = self.bucket.get_object(object_path).await?;
        Ok(response_data.into())

    }

    pub async fn create_object(&self, object_path: &str, object: &[u8]) -> Result<(), anyhow::Error> {

        let _ = self.bucket.put_object(object_path, &object).await?;
        Ok(())

    }

}

pub fn get_full_path_storage(relative_path: Option<String>) -> Option<String> {

    match relative_path {

        Some(relative_path) => {
            return Some(format!("https://{}.{}{}", settings::get("bucket_prefix"), settings::get("storage_endpoint"), relative_path));
        },
        None => None

    }

}

pub fn get_full_path() -> String {

    FULL_PATH.to_string()

}