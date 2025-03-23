#![allow(dead_code)]
use minio_rsc::provider::StaticProvider;
use minio_rsc::Minio;
use actix_web::web::Bytes;

use dotenv::dotenv;
use std::env;
use std::error::Error;

pub struct MinioData {
  pub client: Minio,
  pub bucket_name: String,
}

impl MinioData {
  pub async fn new() -> MinioData {
    dotenv().ok();

    let minio_url = env::var("MINIO_URL").unwrap_or("localhost:9022".to_string());
    let minio_access = env::var
      ("MINIO_ACCESS_KEY").unwrap_or("minio-access-key-test".to_string());
    let minio_secret = env::var
      ("MINIO_SECRET_KEY").unwrap_or("secret-key-test".to_string());
    let bucket_name = env::var("MINIO_BUCKET_NAME").unwrap_or("test".to_string());

    let provider = StaticProvider::new(&minio_access, &minio_secret, None);
    let minio = Minio::builder()
      .endpoint(minio_url)
      .provider(provider)
      .secure(false)
      .build()
      .unwrap();

    MinioData {
      client: minio,
      bucket_name,
    }
  }

  pub async fn upload_file(&self, file_name: &str, data: Bytes) -> Result<(), Box<dyn Error>> {
    let _ = self.client.put_object(self.bucket_name.clone(), file_name, data).await;
    Ok(())
  }

  pub async fn download_file(&self, file_name: &str) -> Result<Bytes, Box<dyn Error>> {
    let buff =  self.client
      .get_object(self.bucket_name.clone(), file_name)
      .await.unwrap()
      .bytes()
      .await.unwrap();
    Ok(buff)
  }

}
