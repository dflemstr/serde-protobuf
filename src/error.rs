//! Common error types for this crate.
use std::fmt;
use std::result;

use protobuf;
use protobuf::wire_format;
use serde;
use thiserror::Error;

/// A result whose error type is `Error`.
pub type Result<A> = result::Result<A, Error>;

/// An error that may occur when dealing with Protobuf.
#[derive(Debug, Error)]
pub enum Error {
    /// A native protobuf error.
    #[error("protobuf error")]
    Protobuf(#[source] protobuf::ProtobufError),
    /// The end of stream was reached.
    #[error("end of stream")]
    EndOfStream,
    /// An unknown enum type was encountered.
    #[error("unknown enum: {name}")]
    UnknownEnum {
        /// The name of the enum.
        name: String,
    },
    /// An unknown enum value was encountered.
    #[error("unknown enum value: {value}")]
    UnknownEnumValue {
        /// The number of the enum value.
        value: i32,
    },
    /// An unknown message type was encountered.
    #[error("unknown message: {name}")]
    UnknownMessage {
        /// The name of the message.
        name: String,
    },
    /// An unexpected wire type was received.
    #[error("bad wire type: {wire_type:?}")]
    BadWireType {
        /// The encountered wire type.
        wire_type: wire_format::WireType,
    },
    /// A default value that can't be parsed was received.
    #[error("bad default value: {default_value:?}")]
    BadDefaultValue {
        /// The default value that couldn't be parsed.
        default_value: String,
    },
    /// Some user-defined error occurred.
    #[error("{message}")]
    Custom {
        /// The user-defined error message.
        message: String,
    },
}

/// A result whose error type is `CompatError`.
pub type CompatResult<A> = result::Result<A, CompatError>;

/// A compatibility error for use with `serde`.
#[derive(Debug, Error)]
pub struct CompatError(#[from] Error);

impl From<protobuf::error::ProtobufError> for Error {
    fn from(e: protobuf::error::ProtobufError) -> Self {
        Error::Protobuf(e)
    }
}

impl CompatError {
    /// Converts this compatibility error into the underlying error.
    pub fn into_error(self) -> Error {
        self.0
    }
}

impl fmt::Display for CompatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl serde::de::Error for CompatError {
    fn custom<T>(msg: T) -> CompatError
    where
        T: fmt::Display,
    {
        CompatError(Error::Custom {
            message: msg.to_string(),
        })
    }
}
