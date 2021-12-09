mod client;
pub mod messages;
mod server;

use std::{fmt::Debug, net::SocketAddr, time::Duration};

use bevy::{core::FixedTimestep, prelude::*, utils::Instant};
use laminar::{Packet, Socket, SocketEvent};
use serde::Serialize;
use thiserror::Error;

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

    pub fn receiver(&self) -> Result<impl IntoIterator<Item = SocketEvent>, NoSocket> {
        let socket = self.socket.as_ref().ok_or(NoSocket)?;
        let receiver = socket.get_event_receiver();
        Ok(receiver)
    }

    pub fn send_message<Message, F>(
        &self,
        address: SocketAddr,
        message: &Message,
        packeter: F,
    ) -> Result<(), NoSocket>
    where
        Message: Serialize,
        F: FnOnce(SocketAddr, Vec<u8>) -> Packet,
    {
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

pub struct NetworkSendEvent<Message> {
    message: Message,
    address: SocketAddr,
    packetting: Packetting,
}

fn message_broadcast<Message>(
    mut message_packet: EventReader<NetworkSendEvent<Message>>,
    network_server: Res<NetworkServer>,
) where
    Message: Serialize + Debug + Sync + Send + 'static,
{
    // TODO: Actually drain this and reduce clones
    for NetworkSendEvent {
        message,
        address,
        packetting,
    } in message_packet.iter()
    {
        let res = network_server.send_message(address.clone(), message, packetting.to_fn());
        if let Err(error) = res {
            error!(message = "failed to broadcast message", %address, ?message, %error);
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
        app.insert_resource(NetworkServer::new(self.address.clone()))
            .add_startup_system(setup_server.system())
            .add_system_set(polling);
    }
}
