use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn db_connection() -> PgConnection {
    let database_url = "postgres://postgres:Xujiyou.19971118@localhost/rust_blog".to_string();
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}