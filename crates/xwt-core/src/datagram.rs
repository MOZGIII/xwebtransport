use async_trait::async_trait;

use crate::utils::maybe;

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait Receive: maybe::Send {
    type Datagram: maybe::Send + AsRef<[u8]>;
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn receive_datagram(&self) -> Result<Self::Datagram, Self::Error>;
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait ReceiveInto: maybe::Send {
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn receive_datagram_into(&self, buf: &mut [u8]) -> Result<usize, Self::Error>;
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait Send: maybe::Send {
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn send_datagram<D>(&self, payload: D) -> Result<(), Self::Error>
    where
        D: maybe::Send + AsRef<[u8]>;
}

pub trait Datagrams: Send + Receive + ReceiveInto {}

impl<T> Datagrams for T where T: Send + Receive + ReceiveInto {}
