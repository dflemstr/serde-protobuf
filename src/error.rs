//! Common error types for this crate.
use std::error;
use std::fmt;
use std::result;

use protobuf;
use protobuf::wire_format;
use serde;

/// A result whose error type is `Error`.
pub type Result<A> = result::Result<A, Error>;

/// An error that may occur when dealing with Protobuf.
#[derive(Debug, Fail)]
pub enum Error {
    /// A native protobuf error.
    #[fail(display = "protobuf error")]
    Protobuf(#[cause] protobuf::ProtobufError),
    /// The end of stream was reached.
    #[fail(display = "end of stream")]
    EndOfStream,
    /// An unknown enum type was encountered.
    #[fail(display = "unknown enum: {}", name)]
    UnknownEnum {
        /// The name of the enum.
        name: String,
    },
    /// An unknown enum value was encountered.
    #[fail(display = "unknown enum value: {}", value)]
    UnknownEnumValue {
        /// The number of the enum value.
        value: i32,
    },
    /// An unknown message type was encountered.
    #[fail(display = "unknown message: {}", name)]
    UnknownMessage {
        /// The name of the message.
        name: String,
    },
    /// An unexpected wire type was received.
    #[fail(display = "bad wire type: {:?}", wire_type)]
    BadWireType {
        /// The encountered wire type.
        wire_type: wire_format::WireType,
    },
    /// A default value that can't be parsed was received.
    #[fail(display = "bad default value: {:?}", default_value)]
    BadDefaultValue {
        /// The default value that couldn't be parsed.
        default_value: String,
    },
    /// Some user-defined error occurred.
    #[fail(display = "{}", message)]
    Custom {
        /// The user-defined error message.
        message: String,
    },
}

/// A result whose error type is `CompatError`.
pub type CompatResult<A> = result::Result<A, CompatError>;

/// A compatibility error for use with `serde`.
#[derive(Debug)]
pub struct CompatError(failure::Compat<Error>);

impl From<protobuf::ProtobufError> for Error {
    fn from(e: protobuf::ProtobufError) -> Self {
        Error::Protobuf(e)
    }
}

impl CompatError {
    /// Converts this compatibility error into the underlying error.
    pub fn into_error(self) -> Error {
        self.0.into_inner()
    }
}

impl fmt::Display for CompatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl error::Error for CompatError {}

impl From<Error> for CompatError {
    fn from(e: Error) -> Self {
        use failure::Fail;
        CompatError(e.compat())
    }
}

impl serde::de::Error for CompatError {
    fn custom<T>(msg: T) -> CompatError
    where
        T: fmt::Display,
    {
        use failure::Fail;
        CompatError(
            Error::Custom {
                message: msg.to_string(),
            }
            .compat(),
        )
    }
}
