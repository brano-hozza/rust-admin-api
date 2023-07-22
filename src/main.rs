use std::net::Ipv4Addr;

mod common;
mod users;

use common::DbPool;

use actix_web::{
    error,
    web::{self, Data},
    App, HttpServer,
};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use users::{create_user, get_users};

pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = std::env::var("DATABASE_HOST").unwrap();
    let port = std::env::var("DATABASE_PORT")
        .unwrap()
        .parse::<u16>()
        .unwrap();
    let database = std::env::var("DATABASE_NAME").unwrap();
    let user = std::env::var("DATABASE_USER").unwrap();
    let password = std::env::var("DATABASE_PASSWORD").unwrap();

    let pool: DbPool = Pool::builder()
        .build(ConnectionManager::new(format!(
            "postgres://{user}:{password}@{host}:{port}/{database}"
        )))
        .unwrap();

    pool.get()
        .unwrap()
        .run_pending_migrations(MIGRATIONS)
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::JsonConfig::default().error_handler(|_, _| error::ErrorBadRequest("")))
            .app_data(web::PathConfig::default().error_handler(|_, _| error::ErrorBadRequest("")))
            .app_data(Data::new(pool.clone()))
            .service(get_users)
            .service(create_user)
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8000))?
    .run()
    .await
}
