use crate::services::ServiceErrorTypes::InfoNotFound;
use serde::Serialize;
use std::fmt;

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
            _ => write!(f, "Service Error: General Error"),
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

pub mod order_service;
pub mod product_service;
pub mod stock_service;
