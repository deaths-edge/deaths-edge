use std::{fmt::Debug, hash::Hash, marker::PhantomData, net::SocketAddr};

use bevy::prelude::*;
use serde::Serialize;

use super::{NetworkServer, Packetting};

pub struct NetworkSendEvent<Message> {
    message: Message,
    address: SocketAddr,
    packetting: Packetting,
}

impl<Message> NetworkSendEvent<Message> {
    pub fn new(message: Message, address: SocketAddr, packetting: Packetting) -> Self {
        Self {
            message,
            address,
            packetting,
        }
    }
}

/// Listens to [`NetworkSendEvent`] and then sends the internal message.
pub fn send_message<Message>(
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

pub struct NetworkSendPlugin<T, Message> {
    state: T,
    _message: PhantomData<Message>,
}

impl<T, Message> NetworkSendPlugin<T, Message> {
    pub fn new(state: T) -> Self {
        Self {
            state,
            _message: PhantomData,
        }
    }
}

impl<T, Message> Plugin for NetworkSendPlugin<T, Message>
where
    Message: Send + Sync + Debug + 'static,
    Message: Serialize,
    T: Send + Sync + Copy + Hash + Debug + Eq + 'static,
{
    fn build(&self, app: &mut AppBuilder) {
        let send_network =
            SystemSet::on_update(self.state).with_system(send_message::<Message>.system());
        app.add_event::<NetworkSendEvent<Message>>()
            .add_system_set(send_network);
    }
}
