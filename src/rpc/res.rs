use serde::{Serialize, Deserialize};
use super::{Error, Code, Version};

/// JSON RPC response
///
/// ```rust
/// use ethgen::rpc::{Response, Code};
///
/// 
/// let json = r#"{
///     "jsonrpc": "2.0",
///     "error": {
///         "code": -32600,
///         "message": "Invalid request"
///     },
///     "id": 1
/// }"#;
///
///
/// let (res, _): (Response<u64>, usize) = 
///     serde_json_core::from_str(json).unwrap();
///
/// assert_eq!(res.error_code(), Some(Code::InvalidRequest));
///
///
///
/// let json = r#"{"jsonrpc":"2.0","result":42,"id":1}"#;
///
/// let (res, _): (Response<u64>, usize) =
///     serde_json_core::from_str(json).unwrap();
///
/// assert_eq!(res.unwrap(), 42);
///
///
///
/// type Batch = (Response<u64>, Response<u64>);
/// let json = r#"[
///     {
///         "jsonrpc": "2.0",
///         "error": {
///             "code": -32600,
///             "message": "Invalid request"
///         },
///         "id": 1
///     },
///     {
///         "jsonrpc": "2.0",
///         "result": 42,
///         "id": 2
///     }
/// ]"#;
///
/// let (res, _) = serde_json_core::from_str::<Batch>(json).unwrap();
/// 
/// assert_eq!(res.0.error_code(), Some(Code::InvalidRequest));
/// assert_eq!(res.1.unwrap(), 42);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response<T> {
    pub jsonrpc: Version,
    pub id: u64, // TODO: make it U256
    pub result: Option<T>,
    pub error: Option<Error>,
}

impl<'de, T> Response<T> {
    pub fn into_result(self) -> Result<T, Error> {
        match self.result {
            Some(result) => Ok(result),
            None => match self.error {
                Some(err) => Err(err),
                None => Err(Error::new(-32700, "unknown")),
            },
        }
    }

    pub fn unwrap(self) -> T {
        match self.into_result() {
            Ok(result) => result,
            Err(err) => panic!("RPC error {:?}", err),
        }
    }

    pub fn error_code(self) -> Option<Code> {
        match self.error {
            Some(err) => Some(err.code),
            None => None,
        }
    }
}
