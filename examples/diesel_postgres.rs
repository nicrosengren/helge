use diesel::r2d2::{ConnectionManager, ManageConnection};
use helge::Helge;

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
#[diesel(table_name = users)]
struct NewUser {
    name: String,
}

#[tokio::main]
async fn main() {
    let database_url = "postgres://localhost/helgeexample";

    let connection_manager = ConnectionManager::<diesel::PgConnection>::new(database_url);
    let mut conn = connection_manager.connect().expect("Connecting");
    diesel::sql_query("DROP TABLE IF EXISTS users")
        .execute(&mut conn)
        .expect("dropping table `users`");

    diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS users(id SERIAL PRIMARY KEY, name VARCHAR NOT NULL)",
    )
    .execute(&mut conn)
    .expect("creating table `users`");

    // Create Helge from a Builder, allowing custimization of the Pool.
    let pool = diesel::r2d2::Builder::new()
        .build(connection_manager)
        .expect("Building r2d2 pool");

    let pool = Helge::from_pool(pool);

    pool.query(|conn| {
        diesel::insert_into(users::table)
            .values(&NewUser {
                name: String::from("Helge"),
            })
            .execute(conn)
    })
    .await
    .expect("inserting users");

    let helge = pool
        .query(|conn| users::table.get_results::<(i32, String)>(conn))
        .await
        .expect("reading users");

    println!("Fetched {:?}", helge);
}
