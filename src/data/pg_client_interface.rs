use sqlx::{postgres::PgPoolOptions, Connection};
use std::result::Result;

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

async fn pg_pool_init() -> Result<(), sqlx::Error> {
    let postgres_password = "Crudux:Cruo_i18n";
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:" + postgres_password + "@localhost/captions")
        .await?; // TODO(Config): save this connection data into a safe JSON/Configfile.

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let mut rows = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ? OR username = ?")
        .bind(email)
        .bind(username)
        .fetch_one(&pool)
        .await?;

    let mut videos = sqlx::query_as::<_, Video>("SELECT * FROM videos WHERE id = ? or id = 0")
        .bind(username)
        .bind(title)
        .fetch_one(&pool)
        .await?;

    while let Some(rows) = rows.try_next().await? {
        let email: &str = rows.try_get("email")?;
        let username: &str = rows.try_get("username")?;
        let title: &str = videos.try_get("title")?;
    }

    Ok(())
}

async fn pg_pool_insert(insertion_args: &str) -> Result<(), sqlx::Error> {
    let mut rows = sqlx::query_as::<_, Video>(insertion_args)
        .bind(username)
        .bind(title)
        .await?;

    while let Some(rows) = rows.try_next().await? {
        let username: &str = rows.try_get("username")?;
        let title: &str = rows.try_get("title")?;
    }

    Ok(())
}

async fn pg_pool_update(update_args: &str) -> Result<(), sqlx::Error> {
    let mut rows = sqlx::query_as::<_, Video>(update_args)
        .bind(username)
        .bind(title)
        .await?;

    while let Some(rows) = rows.try_next().await? {
        let username: &str = rows.try_get("username")?;
        let title: &str = rows.try_get("title")?;
    }

    Ok(())
}

async fn pg_pool_destroy(deletion_args: &str) -> Result<(), sqlx::Error> {
    let mut rows = sqlx::query_as::<_, Video>(deletion_args)
        .bind(username)
        .bind(title)
        .await?;

    while let Some(rows) = rows.try_next().await? {
        let username: &str = rows.try_get("username")?;
        let title: &str = rows.try_get("title")?;
    }

    Ok(())
}
