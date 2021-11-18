//! Helge is a tiny wrapper around r2d2::Pool and diesel ConnectionManager
//! to provide a simple way to use diesel postgres with r2d2 in an async Context.
//!
//! <br>
//!  # Example
//! ```rust
//!
//! let helge = Helge::<diesel::PgConnection>::new("postgres://localhost/somedatabase")?;
//! helge
//!       .query(|conn| {
//!           diesel::insert_into(users::table)
//!               .values(&NewUser {
//!                   name: String::from("Helge"),
//!                })
//!                .execute(conn)
//!        })
//!        .await?;
//!
//! ```

use diesel::r2d2::ConnectionManager;
use r2d2::ManageConnection;

mod error;
pub use error::{ConnectionError, Error};

/// The main wrapper, simply contains an r2d2::Pool
pub struct Helge<C>
where
    C: diesel::Connection + Send + 'static,
{
    pool: r2d2::Pool<diesel::r2d2::ConnectionManager<C>>,
}

impl<C> Clone for Helge<C>
where
    C: diesel::Connection + Send + 'static,
{
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

impl<C> Helge<C>
where
    C: diesel::Connection + Send + 'static,
{
    /// Create a new Helge with default settings
    pub fn new(database_uri: impl Into<String>) -> Result<Self, ConnectionError> {
        let manager = ConnectionManager::<C>::new(database_uri);

        let _ = manager.connect().map_err(|err| match err {
            diesel::r2d2::Error::ConnectionError(err) => ConnectionError::Connection(err),
            diesel::r2d2::Error::QueryError(err) => ConnectionError::PingFailed(err),
        })?;

        let pool = diesel::r2d2::Builder::new().build(manager)?;

        Ok(Self { pool })
    }

    pub fn from_pool(pool: r2d2::Pool<diesel::r2d2::ConnectionManager<C>>) -> Self {
        Self { pool }
    }

    pub fn get_conn(
        &self,
    ) -> Result<r2d2::PooledConnection<diesel::r2d2::ConnectionManager<C>>, Error> {
        self.pool.get().map_err(Error::from)
    }

    pub async fn query<T, F>(&self, f: F) -> std::result::Result<T, Error>
    where
        T: Send + 'static,
        F: FnOnce(&C) -> std::result::Result<T, diesel::result::Error> + Send + 'static,
    {
        let pool = self.pool.clone();

        tokio::task::spawn_blocking(move || {
            let conn = pool.get().map_err(Error::Pool)?;

            f(&conn).map_err(Error::Query)
        })
        .await
        .map_err(Error::Runtime)?
    }

    pub async fn run<T, E, F>(&self, f: F) -> std::result::Result<T, E>
    where
        T: Send + 'static,
        E: From<Error> + Send + 'static,
        F: FnOnce(&C) -> std::result::Result<T, E> + Send + 'static,
    {
        let pool = self.pool.clone();

        tokio::task::spawn_blocking(move || {
            let conn = pool.get().map_err(|err| E::from(Error::Pool(err)))?;

            f(&conn)
        })
        .await
        .map_err(|err: tokio::task::JoinError| E::from(Error::Runtime(err)))?
    }
}
