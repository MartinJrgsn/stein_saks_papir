pub mod error;

use std::{time::SystemTime, ops::Sub};

pub use error::*;

pub trait DeserializeTcp: Sized
{
    fn from_tcp_message(bytes: &[u8]) -> Self;
}

pub trait TryDeserializeTcp: Sized
{
    fn try_from_tcp_message(bytes: &[u8]) -> Result<Self, DeserializeTcpError>;
}

impl<T> TryDeserializeTcp for T
where T: DeserializeTcp
{
    fn try_from_tcp_message(bytes: &[u8]) -> Result<Self, DeserializeTcpError>
    {
        Ok(Self::from_tcp_message(bytes))
    }
}

impl TryDeserializeTcp for std::time::SystemTimeError
{
    fn try_from_tcp_message(bytes: &[u8]) -> Result<Self, DeserializeTcpError>
    {
        const SIZE64: usize = u64::BITS as usize/8;
        const SIZE32: usize = u32::BITS as usize/8;
        const SIZE: usize = SIZE64 + SIZE32;

        let mut iter = bytes.get(0..SIZE)
            .ok_or(DeserializeTcpError::InsufficientBufferLength(bytes.len()))?
            .into_iter();

        let duration = std::time::Duration::new(
        {
            let mut bytes = [0; SIZE64];
            bytes.fill_with(|| *iter.next().unwrap());
            u64::from_le_bytes(bytes)
        }, {
            let mut bytes = [0; SIZE32];
            bytes.fill_with(|| *iter.next().unwrap());
            u32::from_le_bytes(bytes)
        });

        let now = SystemTime::now();
        
        match now.sub(duration).duration_since(now)
        {
            Ok(_) => Err(DeserializeTcpError::DataParseError),
            Err(error) => Ok(error)
        }
    }
}