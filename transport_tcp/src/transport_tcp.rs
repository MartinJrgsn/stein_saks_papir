use std::{io::{BorrowedBuf, BorrowedCursor, Read, Write}, net::{SocketAddr, TcpListener, TcpStream}, os::windows::io::AsSocket, time::Duration};

use atomic_buffer::{AtomicBufferWeak, error::BufferError};
use serde::{Deserialize, Serialize};
use transport::{transport::*, ParaStream, ReceiveBufferShare, ReceiveBuffer};

use super::*;

#[derive(Clone, Copy, Debug)]
pub struct TransportTcp;

impl Transport for TransportTcp
{
    type Target = SocketAddr;

    type MessageError = TcpMessageError;

    const NAME: &'static str = "TCP";
}

impl TransportObj for TransportTcp
{
    
}

impl<RequestType, ResponseType> ListenerTransport<RequestType, ResponseType> for TransportTcp
where
    RequestType: Serialize + Send + Sync + 'static,
    for<'a> ResponseType: Deserialize<'a> + Send + Sync + 'static
{
    type ListenerError = TcpListenerError<Self::Target>;
    type ListenerArgs = TcpListener;
    type SpawnListenerError = SpawnTcpListenerError;

    type ConnectError = TcpConnectError;

    fn bind_listener(target: Self::Target) -> Result<(Self::Target, Self::ListenerArgs), Self::SpawnListenerError>
    {
        /*let ip = local_ip_address::local_ip()?;
        let target = SocketAddr::new(ip, target);*/

        Ok((target, TcpListener::bind(target).map_err(|error| SpawnTcpListenerError::BindError(error))?))
    }

    fn listener_loop(
        transport: &std::sync::Weak<std::sync::RwLock<Self>>,
        buffer_incoming: &AtomicBufferWeak<(Self::Target, Result<ParaStream<RequestType, ResponseType, Self, ReceiveBufferShare<RequestType, ResponseType, Self>>, Self::ConnectError>)>,
        buffer_receive: &AtomicBufferWeak<(Self::Target, Result<ResponseType, Self::MessageError>)>,
        listener: &mut Self::ListenerArgs
    )
        -> Result<(), Self::ListenerError>
    {
        let (stream, peer_addr) = listener.accept()
            .map_err(|error| TcpListenerError::AcceptConnectionError(error))?;

        println!("Accepted {}", peer_addr);

        buffer_incoming.push_back((
            peer_addr,
            match listener.local_addr()
            {
                Ok(target) => match ParaStream::<RequestType, ResponseType, Self, _>::new_from_connection(
                    "Listener connection",
                    target,
                    peer_addr,
                    stream,
                    transport.clone(),
                    ReceiveBufferShare::new(peer_addr, buffer_receive.clone())
                )
                {
                    Ok(para_stream) => Ok(para_stream),
                    Err(error) => Err(TcpConnectError::SpawnTcpStreamError(error))
                },
                Err(error) => Err(TcpConnectError::LocalAddrError(error))
            }
        )).map_err(|error| TcpListenerError::BufferError(error))?;

        Ok(())
    }

    fn missing_connection_error(target: Self::Target) -> Self::ListenerError
    {
        TcpListenerError::MissingConnectionError(target)
    }
}

impl<RequestType, ResponseType> StreamTransport<RequestType, ResponseType> for TransportTcp
where
    RequestType: Serialize,
    for<'a> ResponseType: Deserialize<'a>,
    Self: Clone + Send + Sync
{
    type StreamError = TcpStreamError;
    type StreamArgs = TcpStream;
    type SpawnStreamError = SpawnTcpStreamError;

    fn connect_stream(target: Self::Target) -> Result<(Self::Target, Self::StreamArgs), Self::SpawnStreamError>
    {
        let connection = TcpStream::connect(target).map_err(|error| SpawnTcpStreamError::ConnectError(error))?;

        Ok((connection.peer_addr().map_err(|error| SpawnTcpStreamError::ConnectError(error))?, connection))
    }
    fn stream_loop<B>(
        _transport: &std::sync::Weak<std::sync::RwLock<Self>>,
        buffer_send: &AtomicBufferWeak<RequestType>,
        buffer_receive: &B,
        mut stream: &mut Self::StreamArgs
    )
        -> Result<(), Self::StreamError>
    where
        B: ReceiveBuffer<RequestType, ResponseType, Self>
    {
        while let Some(message) = buffer_send.pop_front()?
        {
            if let Err(error) = write_tcp_message(&mut stream, message)
            {
                buffer_receive.push_back(Err(error.into()))?
            }
        }

        while let Some(message_result) = read_tcp_message(&mut stream)
        {
            buffer_receive.push_back(message_result.map_err(Into::into))?
        }

        Ok(())
    }
}

fn write_tcp_message<MessageType>(stream: &mut TcpStream, message: MessageType) -> Result<(), TcpMessageWriteError>
where
    MessageType: Serialize
{
    let bytes = match bincode::serialize(&message)
    {
        Ok(bytes) => bytes,
        Err(error) => return Err(TcpMessageWriteError::SerializeError(error))
    };

    if let Err(error) = stream.write(&bytes)
    {
        return Err(TcpMessageWriteError::WriteToStreamError(error))
    }

    Ok(())
}

fn read_tcp_message<MessageType>(stream: &mut TcpStream) -> Option<Result<MessageType, TcpMessageReadError>>
where
    for<'a> MessageType: Deserialize<'a>
{
    const TIMEOUT_DURATION: Duration = Duration::new(2, 0);
    stream.set_read_timeout(Some(TIMEOUT_DURATION)).unwrap();
    let mut bytes = vec![];
    match stream.read_to_end(&mut bytes)
    {
        Ok(size) => if size == 0
        {
            None
        }
        else
        {
            println!("Read something");
            match bincode::deserialize(&bytes)
            {
                Ok(message) => Some(Ok(message)),
                Err(error) => Some(Err(TcpMessageReadError::DeserializeError(error)))
            }
        },
        Err(error) => if error.raw_os_error() == Some(10060)
        {
            None
        }
        else
        {
            Some(Err(TcpMessageReadError::ReadFromStreamError(error)))
        }
    }
}