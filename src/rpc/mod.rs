//! RPC

use serde::{Deserialize, Serialize};

mod error;
mod request;
mod response;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Version {
    #[serde(rename = "2.0")]
    V2,
}

#[doc(inline)]
pub use error::Error;
#[doc(hidden)]
pub use error::{Code, Message};

#[doc(hidden)]
pub use request::Method;
#[doc(inline)]
pub use request::Request;

#[doc(inline)]
pub use response::Response;

// TODO: List all available RPC Request factories with accordning Responses.
