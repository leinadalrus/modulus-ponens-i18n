use sqlx::{postgres::PgPoolOptions, Connection};
use std::result::Result;

#[derive(sqlx::FromRow)]
struct User {
    id: i64,
    email: String,
    username: String,
}

async fn pg_pool_init() -> Result<(), sqlx::Error> {
    let postgres_password = "Crudux:Cruo_i18n";
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:" + postgres_password + "@localhost/captions")
        .await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let mut rows = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ? OR username = ?")
        .bind(email)
        .bind(username)
        .fetch_one(&pool)
        .await?;

    while let Some(rows) = rows.try_next().await? {
        let email: &str = rows.try_get("email")?;
        let username: &str = rows.try_get("username")?;
    }

    Ok(())
}
