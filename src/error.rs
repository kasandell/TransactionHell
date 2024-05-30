use std::fmt::Debug;
use std::num::ParseIntError;
use diesel_async::pooled_connection::bb8::RunError;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use r2d2::Error as R2D2Error;
use serde_json::{json, Error as SerdeError};
use thiserror;


#[derive(thiserror::Error, Debug)]
pub enum DataError {
    #[error("Unexpected error")]
    Unexpected(#[source] Box<dyn std::error::Error + Send>),
}

impl From<R2D2Error> for DataError {
    fn from(e: R2D2Error) -> DataError {
        DataError::Unexpected(Box::new(e))
    }
}

impl From<RunError> for DataError {
    fn from(error: RunError) -> DataError {
        DataError::Unexpected(Box::new(error))
    }

}

impl From<DieselError> for DataError {
    fn from(error: DieselError) -> DataError {
        DataError::Unexpected(Box::new(error))
    }
}
