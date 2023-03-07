#![feature(decl_macro)]
#[macro_use] extern crate rocket;
use rocket::{
    fairing::{self, AdHoc},
    futures,
    response::status::Created,
    serde::{json::Json, Deserialize, Serialize},
    Build, Rocket,
};
use rocket_db_pools::{sqlx, Connection, Database};
extern crate sqlx as external_sqlx;
use std::result::Result;
use external_sqlx::postgres::PgPoolOptions;

#[derive(Database)]
#[database("sqlx")]
struct PgDatabase(sqlx::PgPool);

#[derive(sqlx::FromRow)]
struct User {
    id: i64,
    email: String,
    username: String,
}

#[derive(sqlx::FromRow)]
struct Caption;

#[derive(sqlx::FromRow)]
struct Video {
    id: i64,
    user: User,
    title: String,
    captions: Caption,
}

async fn pg_pool_init() -> Result<(), external_sqlx::Error> {
    let postgres_password = "Crudux:Cruo_i18n";
    let pool = PgPoolOptions::new().max_connections(1).connect(
        &"postgres://postgres:ehm4rn3ndx_wky3xep@localhost/captions".to_owned(), // Fake password
    ); // TODO(Config): save this connection data into a safe JSON/Configfile.

    // Make a simple query to return the given parameter (use a question mark
    // `?` instead of `$1` for MySQL)
    let mut rows = external_sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = ? OR username = ?",
    )
    .bind(email)
    .bind(username);

    let mut videos = external_sqlx::query_as::<_, Video>(
        "SELECT * FROM videos WHERE id = ? or id = 0",
    )
    .bind(username)
    .bind(title);

    while let Ok(Some(rows)) = rows.try_into() {
        let email: &str = "email";
        let username: &str = "username";
    }

    Ok(())
}

async fn pg_pool_insert(insertion_args: &str) -> Result<(), external_sqlx::Error> {
    let mut rows = external_sqlx::query_as::<_, Video>(insertion_args)
        .bind(user)
        .bind(title);

    while let Ok(Some(rows)) = rows.try_into() {
        let user: &str = "user";
        let title: &str = "title";
    }

    Ok(())
}

async fn pg_pool_update(update_args: &str) -> Result<(), external_sqlx::Error> {
    let mut rows = external_sqlx::query_as::<_, Video>(update_args)
        .bind(user)
        .bind(title);

    while let Ok(Some(rows)) = rows.try_into() {
        let user: &str = "user";
        let title: &str = "title";
    }

    Ok(())
}

async fn pg_pool_destroy(deletion_args: &str) -> Result<(), external_sqlx::Error> {
    let mut rows = external_sqlx::query_as::<_, Video>(deletion_args)
        .bind(user)
        .bind(title);

    while let Ok(Some(rows)) = rows.try_into() {
        let user: &str = "user";
        let title: &str = "title";
    }

    Ok(())
}

async fn pg_migrations_run(rocket: Rocket<Build>) -> fairing::Result {
    match Database::fetch(&rocket) {
        Some(db) => match external_sqlx::migrate!("db/sqlx/migrations") {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },

        None => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket
            .attach(Database::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", pg_migrations_run))
    })
}
