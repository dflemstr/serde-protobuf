use std::error;
use std::fmt;
use std::result;

use protobuf;
use protobuf::stream::wire_format;
use serde;

pub type Result<A> = result::Result<A, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "protobuf error")]
    Protobuf(protobuf::ProtobufError),
    #[fail(display = "end of stream")]
    EndOfStream,
    #[fail(display = "unknown enum: {}", name)]
    UnknownEnum { name: String },
    #[fail(display = "unknown enum value: {}", value)]
    UnknownEnumValue { value: i32 },
    #[fail(display = "unknown message: {}", name)]
    UnknownMessage { name: String },
    #[fail(display = "bad wire type: {:?}", wire_type)]
    BadWireType { wire_type: wire_format::WireType },
    #[fail(display = "bad default value: {:?}", default_value)]
    BadDefaultValue { default_value: String },
    #[fail(display = "{}", message)]
    Custom { message: String },
}

pub type CompatResult<A> = result::Result<A, CompatError>;

#[derive(Debug)]
pub struct CompatError(failure::Compat<Error>);

impl From<protobuf::error::ProtobufError> for Error {
    fn from(e: protobuf::error::ProtobufError) -> Self {
        Error::Protobuf(e)
    }
}

impl CompatError {
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
        where T: fmt::Display
    {
        use failure::Fail;
        CompatError(Error::Custom { message: msg.to_string() }.compat())
    }
}
