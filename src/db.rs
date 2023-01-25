

use tokio;
use postgres::{Client, NoTls};
use tokio_postgres;
use crate::settings;

pub fn get_connection() -> Result<postgres::Client, postgres::Error> {

    Client::connect(&settings::get("postgres_connection"), NoTls)
    
}

pub async fn get_async_connection() -> Result<tokio_postgres::Client, tokio_postgres::Error> {

     let (client, connection) = tokio_postgres::connect(&settings::get("postgres_connection"), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

     Ok(client)

}