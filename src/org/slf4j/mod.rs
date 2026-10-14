//Imports
use std::error::Error;

// module
pub mod events;
pub mod spi;
pub mod helpers;

// file
mod marker;
mod i_marker_factory;
mod marker_factory;
mod logger_factory;

// use
pub use marker::{Marker, MarkerConst};
pub use i_marker_factory::IMarkerFactory;
pub use marker_factory::MarkerFactory;
pub use logger_factory::LoggerFactory;

//types
pub trait ErrorType:Error + Send + Sync {}