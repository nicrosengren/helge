use helge::Helge;

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

    let helge = Helge::<diesel::PgConnection>::new(database_url).expect("creating Helge");

    let mut conn = helge.get_conn().expect("getting connection");

    diesel::sql_query("DROP TABLE IF EXISTS users")
        .execute(&mut conn)
        .expect("dropping table `users`");
    diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS users(id SERIAL PRIMARY KEY, name VARCHAR NOT NULL)",
    )
    .execute(&mut conn)
    .expect("creating table `users`");

    helge
        .query(|conn| {
            diesel::insert_into(users::table)
                .values(&NewUser {
                    name: String::from("Helge"),
                })
                .execute(conn)
        })
        .await
        .expect("inserting users");

    let user = helge
        .query(|conn| users::table.get_results::<(i32, String)>(conn))
        .await
        .expect("reading users");

    println!("Fetched {:?}", user);
}
