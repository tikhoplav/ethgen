use serde::{Serialize, Deserialize};
use super::{Version, Error, Code};

/// RPC response
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
				None => Err(Error::new(Code::ParseError, "unknown".into())),
			}
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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn error_response_from_json() {
		let json = r#"{"jsonrpc":"2.0","error":{"code":-32600,"message":"Invalid request"},"id":1}"#;
		let (result, _) = serde_json_core::from_str::<Response<u64>>(json).unwrap();

		assert_eq!(result.error_code(), Some(Code::InvalidRequest));
	}

	#[test]
	fn result_response_from_json() {
		let json = r#"{"jsonrpc":"2.0","result":42,"id":1}"#;
		let (result, _) = serde_json_core::from_str::<Response<u64>>(json).unwrap();

		assert_eq!(result.unwrap(), 42);
	}

	#[test]
	fn batch_repsonse_from_json() {
		type Batch = (Response<u64>, Response<u64>);
		let json = r#"[{"jsonrpc":"2.0","error":{"code":-32600,"message":"Invalid request"},"id":1},{"jsonrpc":"2.0","result":42,"id":2}]"#;
		let (result, _) = serde_json_core::from_str::<Batch>(json).unwrap();

		assert_eq!(result.0.error_code(), Some(Code::InvalidRequest));
		assert_eq!(result.1.unwrap(), 42);
	}
}
