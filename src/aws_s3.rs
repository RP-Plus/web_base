
use s3::{Bucket, Region};
use s3::creds::Credentials;
use anyhow::Result;

use crate::settings;

pub struct Space {

    bucket: Bucket

}

impl Space {

    pub fn new() -> Space {

        let bucket_name = settings::get_setting("bucket_prefix");
        let region      = Region::Custom { region: "us-east-1".to_string(), endpoint: settings::get_setting("storage_endpoint") };
        let credentials = Credentials::new(Some(&settings::get_setting("storage_key")), Some(&settings::get_setting("storage_secret")), None, None, None).unwrap();
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

        let (data, _code) = self.bucket.get_object(object_path).await?;
        Ok(data)

    }

    pub async fn create_object(&self, object_path: &str, object: &[u8]) -> Result<(), anyhow::Error> {

        let (_, _) = self.bucket.put_object(object_path, &object).await?;
        Ok(())

    }

}

pub fn get_full_path_storage(relative_path: Option<String>) -> Option<String> {

    match relative_path {

        Some(relative_path) => {
            return Some(format!("https://{}.{}{}", settings::get_setting("bucket_prefix"), settings::get_setting("storage_endpoint"), relative_path));
        },
        None => None

    }

}