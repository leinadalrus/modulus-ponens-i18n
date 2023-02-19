use sqlx::{
    Connection,
    postgres::PgPoolOptions
};
use std::result::Result;

async fn pg_pool_init() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:password@localhost/captions").await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: (u8,) = sqlx::query_as("SELECT $1")
        .bind(1_u8)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 1);

    Ok(())
}