//! JSON RPC
//!
//! Contains helper functions, factories and data structures to create and
//! serialize RPC requests and deserialize responses to operational data,
//! based on the current Ethereum JSON RPC specifications:
//! <https://ethereum.org/en/developers/docs/apis/json-rpc/>

use serde::{Deserialize, Serialize};

/// JSON RPC version
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Version {
    #[serde(rename = "2.0")]
    V2,
}

mod error;
#[doc(inline)]
pub use error::{Code, Error, Message};

mod res;
#[doc(inline)]
pub use res::Response;

mod req;
#[doc(inline)]
pub use req::{Method, Request};
