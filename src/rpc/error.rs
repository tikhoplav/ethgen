use core::{fmt, str};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// JSON RPC error code
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

/// JSON RPC error message
///
/// Due to `no_std` restrictions the message is presented as a byte array with
/// constant size containing utf8 bytes of the error message. Currently **128**
/// bytes is used to store the error message (this amount may be changed in
/// further versions.
#[derive(PartialEq, Clone)]
pub struct Message([u8; 128], usize);

impl From<&str> for Message {
    #[inline(always)]
    fn from(value: &str) -> Self {
        let mut buf = [0u8; 128];
        let bytes = value.as_bytes();
        bytes.iter().zip(buf.iter_mut()).for_each(|(src, dst)| {
            *dst = *src;
        });
        Self(buf, 128.min(bytes.len()))
    }
}

impl<'a> Into<&'a str> for &'a Message {
    #[inline(always)]
    fn into(self) -> &'a str {
        unsafe { str::from_utf8_unchecked(&self.0[..self.1]) }
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&Self as Into<&str>>::into(self))
    }
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'a> Deserialize<'a> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Ok(s.into())
    }
}

/// JSON RPC error
///
/// ```rust
/// use ethgen::rpc::Error;
///
///
///
/// let err = Error::new(-32600i64, "Invalid request");
///
/// let mut buf = [0u8; 64];
/// let n = serde_json_core::to_slice(&err, &mut buf).unwrap();
/// let json = core::str::from_utf8(&buf[..n]).unwrap();
///
/// let expected = r#"{"code":-32600,"message":"Invalid request"}"#;
/// assert_eq!(expected, json);
///
///
///
/// let json = r#"{"code":-32003,"message":"Call reverted: assertion failed"}"#;
///
/// let (result, _) = serde_json_core::from_str::<Error>(json).unwrap();
/// let expected = Error::new(-32003, "Call reverted: assertion failed");
///
/// assert_eq!(expected, result);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Error {
    pub code: Code,
    pub message: Message,
}

impl Error {
    pub fn new(code: i64, msg: &str) -> Self {
        Self {
            code: code.into(),
            message: msg.into(),
        }
    }
}
