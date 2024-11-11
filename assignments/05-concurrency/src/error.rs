use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum ThreadPoolError {
    #[error("Cannot create thread pool with zero threads")]
    ZeroThreads,

    #[error("Failed to send job to thread pool")]
    SendError,

    #[error("Thread pool poisoned: {0}")]
    PoisonError(String),

    #[error("Thread panicked: {0}")]
    ThreadPanic(String),

    #[error("Failed to receive result: {0}")]
    ReceiveError(String),
}
