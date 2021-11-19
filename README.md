# Helge

Helge is a tiny wrapper around r2d2::Pool and diesel ConnectionManager
to provide a simple way to use diesel postgres with r2d2 in an async Context.

<br>
 # Example
```rust

let helge = Helge::<diesel::PgConnection>::new("postgres://localhost/somedatabase")?;
helge
      .query(|conn| {
          diesel::insert_into(users::table)
              .values(&NewUser {
                  name: String::from("Helge"),
               })
               .execute(conn)
       })
       .await?;

```
