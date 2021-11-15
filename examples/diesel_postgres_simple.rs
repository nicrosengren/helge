use anyhow::Error;
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

    let helge = Helge::<diesel::PgConnection>::new(database_url)?;

    let conn = helge.get_conn()?;

    diesel::sql_query("DROP TABLE IF EXISTS users").execute(&conn)?;
    diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS users(id SERIAL PRIMARY KEY, name VARCHAR NOT NULL)",
    )
    .execute(&conn)?;

    helge
        .query(|conn| {
            diesel::insert_into(users::table)
                .values(&NewUser {
                    name: String::from("Helge"),
                })
                .execute(conn)
        })
        .await?;

    let user = helge
        .query(|conn| users::table.get_results::<(i32, String)>(conn))
        .await?;

    println!("Fetched {:?}", user);

    Ok(())
}
