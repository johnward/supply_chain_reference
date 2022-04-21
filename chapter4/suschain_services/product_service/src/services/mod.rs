use crate::services::ServiceErrorTypes::InfoNotFound;
use serde::Serialize;
use std::fmt;

pub mod product_service;
#[derive(Debug, Serialize)]
pub struct ServiceError {
    //code: usize,
    //message: String,
    error: ServiceErrorTypes,
}

// Implement std::fmt::Display for AppError
// Different error messages according to AppError.code
impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.error {
            InfoNotFound(m) => write!(f, "Service Error: {{ Error Message: {}, }}", m),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum ServiceErrorTypes {
    InfoNotFound(String),
}

fn create_error<T>(error: ServiceErrorTypes) -> Result<T, ServiceError> {
    Err(ServiceError { error })
}
