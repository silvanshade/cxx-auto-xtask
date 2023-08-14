#![deny(clippy::all)]
#![deny(unsafe_code)]

pub mod command;
pub mod config;
pub mod detection;
pub mod handler;
pub mod install;
pub mod validation;
pub mod workspace;

pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type BoxResult<T> = Result<T, BoxError>;

pub use pico_args;
