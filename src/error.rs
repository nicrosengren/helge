/// General Errors that can occur when running queries using Helge.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Query Error: {0}")]
    Query(#[from] diesel::result::Error),

    #[error("Runtime Error. Failed to join blocking thread: {0}")]
    Runtime(#[from] tokio::task::JoinError),

    #[error("Pool Error {0}")]
    Pool(#[from] r2d2::Error),
}

/// ConnectionError can only occur when creating Helge.
#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("Connection failed: {0}")]
    Connection(diesel::result::ConnectionError),

    #[error("Could not Ping database: {0}")]
    PingFailed(diesel::result::Error),

    #[error("Could not create Connection Pool: {0}")]
    PoolSettings(#[from] r2d2::Error),
}
