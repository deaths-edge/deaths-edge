pub mod client;
pub mod server;

use std::{fmt::Debug, time::Duration};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::character::CharacterIndex;

pub use bevy_networking_turbulence::*;

use client::ClientMessage;
use server::ServerMessage;

pub const SERVER_PORT: u16 = 14192;

const CLIENT_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 0,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 4096,
            recv_window_size: 1024,
            send_window_size: 1024,
            burst_bandwidth: 1024,
            init_send: 512,
            wakeup_time: Duration::from_millis(100),
            initial_rtt: Duration::from_millis(200),
            max_rtt: Duration::from_secs(2),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 1024,
    },
    message_buffer_size: 8,
    packet_buffer_size: 8,
};

const SERVER_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 1,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 4096,
            recv_window_size: 1024,
            send_window_size: 1024,
            burst_bandwidth: 1024,
            init_send: 512,
            wakeup_time: Duration::from_millis(100),
            initial_rtt: Duration::from_millis(200),
            max_rtt: Duration::from_secs(2),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 1024,
    },
    message_buffer_size: 8,
    packet_buffer_size: 8,
};

pub const NETWORK_SETUP_LABEL: &str = "network-setup";

pub fn network_setup(mut net: ResMut<NetworkResource>) {
    net.set_channels_builder(|builder| {
        builder
            .register::<ServerMessage>(SERVER_MESSAGE_SETTINGS)
            .unwrap();
        builder
            .register::<ClientMessage>(CLIENT_MESSAGE_SETTINGS)
            .unwrap();
    })
}

/// A character command, addressed by [`CharacterIndex`].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterNetworkCommand<T> {
    pub index: CharacterIndex,
    pub command: T,
}
