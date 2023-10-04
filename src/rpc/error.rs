use core::{fmt, str};
use bytes::Bytes;
use serde::{
	Serialize,
	Serializer,
	Deserialize,
	Deserializer,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Code {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
    TransactionRejected,
    ExecutionError,
    ServerError(i64),
}

impl Code {
    pub fn code(&self) -> i64 {
        match *self {
            Self::ParseError => -32700,
            Self::InvalidRequest => -32600,
            Self::MethodNotFound => -32601,
            Self::InvalidParams => -32602,
            Self::InternalError => -32603,
            Self::TransactionRejected => -32003,
            Self::ExecutionError => 3,
            Self::ServerError(c) => c,
        }
    }
}

impl From<i64> for Code {
    fn from(code: i64) -> Self {
        match code {
            -32700 => Self::ParseError,
            -32600 => Self::InvalidRequest,
            -32601 => Self::MethodNotFound,
            -32602 => Self::InvalidParams,
            -32603 => Self::InternalError,
            -32003 => Self::TransactionRejected,
            3 => Self::ExecutionError,
            _ => Self::ServerError(code),
        }
    }
}

impl Serialize for Code {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.code())
    }
}

impl<'a> Deserialize<'a> for Code {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        i64::deserialize(deserializer).map(Into::into)
    }
}

#[derive(PartialEq, Clone)]
pub struct Message(Bytes);

impl Message {
    pub fn new(bytes: Bytes) -> Self {
        Self(bytes)
    }
}

impl fmt::Debug for Message {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		self.0.fmt(f)
	}
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
    	let s = str::from_utf8(&self.0).unwrap();
        serializer.serialize_str(s)
    }
}

impl<'a> Deserialize<'a> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        let b = Bytes::copy_from_slice(s.as_bytes());
        Ok(Self(b))
    }
}

/// RPC error
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Error {
	pub code: Code,
	pub message: Message,
}

impl Error {
    pub fn new(code: Code, bytes: Bytes) -> Self {
        Self { code, message: Message::new(bytes) }
    }
}

#[cfg(test)]
mod test {
	use super::*;
	use bytes::{BytesMut, BufMut};
	use serde_json_core;

	#[test]
	fn message_into_json() {
		let mut b = BytesMut::with_capacity(64);
		b.put(&b"Hello, this is a little RPC error"[..]);
		let msg = Message(b.into());

		let mut buf = [0u8; 64];
		let n = serde_json_core::to_slice(&msg, &mut buf).unwrap();
		let result = core::str::from_utf8(&buf[..n]).unwrap();
		
		let expected = r#""Hello, this is a little RPC error""#;

		assert_eq!(expected, result);
	}

	#[test]
	fn message_from_json() {
		let json = r#""This is the PRC error message as a custom json string.""#;
		let (result, _) = serde_json_core::from_str::<Message>(json).unwrap();

		let expected = Message(
			Bytes::from("This is the PRC error message as a custom json string.")
		);

		assert_eq!(expected, result);
	}

    #[test]
    fn error_into_json() {
        let err = Error {
            code: Code::InvalidRequest,
            message: Message(Bytes::from("Invalid request")),
        };

        let mut buf = [0u8; 64];
        let n = serde_json_core::to_slice(&err, &mut buf).unwrap();
        let result = core::str::from_utf8(&buf[..n]).unwrap();

        let expected = r#"{"code":-32600,"message":"Invalid request"}"#;

        assert_eq!(expected, result);
    }

    #[test]
    fn error_from_json() {
        let json = r#"{"code":-32003,"message":"Call reverted: assertion failed"}"#;
        let (result, _) = serde_json_core::from_str::<Error>(json).unwrap();

        let expected = Error {
            code: Code::TransactionRejected,
            message: Message(Bytes::from("Call reverted: assertion failed"))
        };

        assert_eq!(expected, result);
    }
}
