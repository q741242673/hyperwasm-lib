//! Serializer implementations for messages.
use thiserror::Error;

use crate::host::api::message;

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("serialization to Bincode failed: {0}")]
    Bincode(#[from] bincode::Error),
    #[cfg(feature = "msgpack_serializer")]
    #[cfg_attr(docsrs, doc(cfg(feature = "msgpack_serializer")))]
    #[error("serialization to MessagePack failed: {0}")]
    MessagePack(#[from] rmp_serde::encode::Error),
    #[cfg(feature = "json_serializer")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json_serializer")))]
    #[error("serialization to Json failed: {0}")]
    Json(#[from] serde_json::error::Error),
    #[cfg(feature = "protobuf_serializer")]
    #[cfg_attr(docsrs, doc(cfg(feature = "protobuf_serializer")))]
    #[error("serialization to Protocol Buffers failed: {0}")]
    ProtocolBuffers(#[from] protobuf::Error),
    #[error("serialization failed: {0}")]
    IO(#[from] std::io::Error),
    #[error("serialization failed: {0}")]
    Custom(String),
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("deserialization from Bincode failed: {0}")]
    Bincode(#[from] bincode::Error),
    #[cfg(feature = "msgpack_serializer")]
    #[cfg_attr(docsrs, doc(cfg(feature = "msgpack_serializer")))]
    #[error("deserialization from MessagePack failed: {0}")]
    MessagePack(#[from] rmp_serde::decode::Error),
    #[cfg(feature = "json_serializer")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json_serializer")))]
    #[error("deserialization from Json failed: {0}")]
    Json(#[from] serde_json::error::Error),
    #[cfg(feature = "protobuf_serializer")]
    #[cfg_attr(docsrs, doc(cfg(feature = "protobuf_serializer")))]
    #[error("deserialization from Protocol Buffers failed: {0}")]
    ProtocolBuffers(#[from] protobuf::Error),
    #[error("serialization failed: {0}")]
    IO(#[from] std::io::Error),
    #[error("deserialization failed: {0}")]
    Custom(String),
}


pub trait Serializer<M> {
    fn encode(message: &M) -> Result<(), EncodeError>;
    fn decode() -> Result<M, DecodeError>;
}


#[derive(Hash, Debug)]
pub struct Bincode {}

impl<M> Serializer<M> for Bincode
where
    M: serde::Serialize + serde::de::DeserializeOwned,
{
    fn encode(message: &M) -> Result<(), EncodeError> {
        Ok(bincode::serialize_into(MessageRw {}, message)?)
    }

    fn decode() -> Result<M, DecodeError> {
        Ok(bincode::deserialize_from(MessageRw {})?)
    }
}


#[cfg(feature = "msgpack_serializer")]
#[cfg_attr(docsrs, doc(cfg(feature = "msgpack_serializer")))]
#[derive(Debug, Hash)]
pub struct MessagePack {}

#[cfg(feature = "msgpack_serializer")]
#[cfg_attr(docsrs, doc(cfg(feature = "msgpack_serializer")))]
impl<M> Serializer<M> for MessagePack
where
    M: serde::Serialize + serde::de::DeserializeOwned,
{
    fn encode(message: &M) -> Result<(), EncodeError> {
        use std::io::Write;
        let data = rmp_serde::to_vec(message)?;
        Ok(MessageRw {}.write_all(&data)?)
    }

    fn decode() -> Result<M, DecodeError> {
        Ok(rmp_serde::decode::from_read(MessageRw {})?)
    }
}


#[cfg(feature = "json_serializer")]
#[cfg_attr(docsrs, doc(cfg(feature = "json_serializer")))]
#[derive(Debug, Hash)]
pub struct Json {}

#[cfg(feature = "json_serializer")]
#[cfg_attr(docsrs, doc(cfg(feature = "json_serializer")))]
impl<M> Serializer<M> for Json
where
    M: serde::Serialize + serde::de::DeserializeOwned,
{
    fn encode(message: &M) -> Result<(), EncodeError> {
        use std::io::Write;
        let data = serde_json::to_vec(message)?;
        Ok(MessageRw {}.write_all(&data)?)
    }

    fn decode() -> Result<M, DecodeError> {
        Ok(serde_json::from_reader(MessageRw {})?)
    }
}


#[cfg(feature = "protobuf_serializer")]
#[cfg_attr(docsrs, doc(cfg(feature = "protobuf_serializer")))]
#[derive(Debug, Hash)]
pub struct ProtocolBuffers {}

#[cfg(feature = "protobuf_serializer")]
#[cfg_attr(docsrs, doc(cfg(feature = "protobuf_serializer")))]
impl<M> Serializer<M> for ProtocolBuffers
where
    M: protobuf::Message,
{
    fn encode(message: &M) -> Result<(), EncodeError> {
        use std::io::Write;
        let mut data = Vec::new();
        message.write_to_vec(&mut data)?;
        Ok(MessageRw {}.write_all(&data)?)
    }

    fn decode() -> Result<M, DecodeError> {
        Ok(M::parse_from_reader(&mut MessageRw {})?)
    }
}


#[derive(Debug, Hash)]
pub struct MessageRw {}

impl std::io::Read for MessageRw {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Ok(unsafe { message::read_data(buf.as_mut_ptr(), buf.len()) })
    }
}

impl std::io::Write for MessageRw {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(unsafe { message::write_data(buf.as_ptr(), buf.len()) })
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
