#![warn(clippy::all, clippy::nursery)]
pub mod error;

pub mod prelude {
    use crate::error::InternalError;

    pub type Result<T> = core::result::Result<T, InternalError>;
}
