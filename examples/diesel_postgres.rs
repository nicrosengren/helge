use anyhow::Error;
use diesel::r2d2::{ConnectionManager, ManageConnection};
use helge::Helge;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;

#[rustfmt::skip]
table! {
    users {
	id -> Integer,
	name -> Varchar,
    }
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
struct NewUser {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let database_url = "postgres://localhost/helgeexample";

    let connection_manager = ConnectionManager::<diesel::PgConnection>::new(database_url);
    let conn = connection_manager.connect()?;

    diesel::sql_query("DROP TABLE IF EXISTS users").execute(&conn)?;
    diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS users(id SERIAL PRIMARY KEY, name VARCHAR NOT NULL)",
    )
    .execute(&conn)?;

    // Create Helge from a Builder, allowing custimization of the Pool.
    let pool = diesel::r2d2::Builder::new().build(connection_manager)?;

    let pool = Helge::from_pool(pool);

    pool.query(|conn| {
        diesel::insert_into(users::table)
            .values(&NewUser {
                name: String::from("Helge"),
            })
            .execute(conn)
    })
    .await?;

    let helge = pool
        .query(|conn| users::table.get_results::<(i32, String)>(conn))
        .await?;

    println!("Fetched {:?}", helge);

    Ok(())
}
