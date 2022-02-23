use static_assertions::assert_obj_safe;
use std::pin::Pin;

use crate::rpc::error::TransportResult;
use crate::rpc::{Datagram, DynFuture};

pub type TransportHandler<'a> =
    Pin<Box<dyn Send + Sync + Fn(&'a Datagram<'a>) -> Option<DynFuture<'a>>>>;

/// Transport protocol has a few main responsibilities:
/// 1. interpreting the raw stream as datagrams
/// 2. automatically reconnecting and dealing with network issues
/// 3. deciding how it wants to handle recv futures
///
/// The transport should not care what the data being sent is nor whether it is valid in the context
/// of the RPC service definitions. It only cares that the packets conform to the datagram
/// specification.
///
/// The `Datagram` type is autogenerated by the bebop compiler when you have at least one service
/// defined. There is no need to make a custom datagram structure.
pub trait TransportProtocol: Send + Sync {
    /// This should only be called once by the Router during initial setup.
    ///
    /// This will await handling calls against our API which can be used to create backpressure or
    /// be ignored. Make sure that you are careful about awaiting these promises or spawning them
    /// on the runtime. Ideally if too many requests are being handled the transport can send
    /// some sort of error response to reject additional work.
    fn set_handler(&mut self, recv: TransportHandler);

    /// Send a datagram to the remote.
    fn send(&self, datagram: &Datagram) -> DynFuture<TransportResult>;
}

assert_obj_safe!(TransportProtocol);
