pub mod client;
mod send;
pub mod server;

pub use send::*;

use std::{fmt::Debug, net::SocketAddr, time::Duration};

use bevy::{core::FixedTimestep, prelude::*, utils::Instant};
use laminar::Socket;
pub use laminar::{Packet, SocketEvent};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::character::CharacterIndex;

pub struct NetworkServer {
    addr: SocketAddr,
    socket: Option<Socket>,
}

#[derive(Debug, Error)]
#[error("no socket connection has been made")]
pub struct NoSocket;

impl NetworkServer {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr, socket: None }
    }

    pub fn poll(&mut self, instant: Instant) -> Result<(), NoSocket> {
        let socket = self.socket.as_mut().ok_or(NoSocket)?;
        socket.manual_poll(instant);
        Ok(())
    }

    pub fn recv(&mut self) -> Result<Option<SocketEvent>, NoSocket> {
        let socket = self.socket.as_mut().ok_or(NoSocket)?;
        Ok(socket.recv())
    }

    pub fn send_message<Message, F>(
        &self,
        address: SocketAddr,
        message: &Message,
        packeter: F,
    ) -> Result<(), NoSocket>
    where
        Message: Serialize + Debug,
        F: FnOnce(SocketAddr, Vec<u8>) -> Packet,
    {
        info!(message = "sending message", payload = ?message, %address);
        let socket = self.socket.as_ref().ok_or(NoSocket)?;
        let sender = socket.get_packet_sender();
        let message_payload = postcard::to_stdvec(message).expect("serialization failed");
        let packet = packeter(address, message_payload);
        sender.send(packet).expect("cannot fail");
        Ok(())
    }
}

pub enum Packetting {
    Unreliable,
    UnreliableOrdered,
}

impl Packetting {
    pub fn to_fn(&self) -> impl FnOnce(SocketAddr, Vec<u8>) -> Packet {
        match self {
            Self::Unreliable => Packet::unreliable,
            Self::UnreliableOrdered => {
                |addr, payload| Packet::unreliable_sequenced(addr, payload, None)
            }
        }
    }
}

fn setup_server(mut network_server: ResMut<NetworkServer>) {
    info!(message = "server binding", addr = %network_server.addr);
    let socket = Socket::bind(network_server.addr).expect("unable to bind");
    network_server.socket = Some(socket);
}

fn poll(time: Res<Time>, mut network_server: ResMut<NetworkServer>) {
    let now = time.last_update().expect("last update not found");
    network_server
        .poll(now)
        .expect("connection not initialized");
}

#[derive(Debug, Clone)]
pub struct NetworkPlugin {
    address: SocketAddr,
    poll_interval: f64,
}

impl NetworkPlugin {
    pub fn new(address: SocketAddr, poll_interval: Duration) -> Self {
        Self {
            address,
            poll_interval: poll_interval.as_secs_f64(),
        }
    }
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let polling = SystemSet::new()
            .with_run_criteria(FixedTimestep::step(self.poll_interval))
            .with_system(poll.system());
        app.insert_resource(NetworkServer::new(self.address))
            .add_startup_system(setup_server.system())
            .add_system_set(polling);
    }
}

/// A character command, addressed by [`CharacterIndex`].
#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterNetworkCommand<T> {
    pub index: CharacterIndex,
    pub command: T,
}
