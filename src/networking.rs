use std::net::SocketAddr;

use bevy::prelude::*;
use bevy_ggrs::RollbackIdProvider;
use ggrs::{P2PSession, PlayerHandle, PlayerType};

use crate::{character::CharacterIndex, Opts};

pub fn start_p2p_session(mut p2p_sess: Option<ResMut<P2PSession>>, opts: Res<Opts>) {
    if let Some(mut p2p_sess) = p2p_sess {
        let mut local_handle = 0;
        let num_players = p2p_sess.num_players() as usize;

        // add players
        for (i, player_addr) in opts.players.iter().enumerate() {
            // local player
            if player_addr == "localhost" {
                p2p_sess.add_player(PlayerType::Local, i).unwrap();
                local_handle = i;
            } else {
                // remote players
                let remote_addr: SocketAddr =
                    player_addr.parse().expect("Invalid remote player address");
                p2p_sess
                    .add_player(PlayerType::Remote(remote_addr), i)
                    .unwrap();
            }
        }

        // set input delay for the local player
        p2p_sess.set_frame_delay(2, local_handle).unwrap();

        // start the GGRS session
        p2p_sess.start_session().unwrap();
    } else {
        error!("p2p_sess not found")
    }
}

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;

// you need to provide a system that represents your inputs as a byte vector, so GGRS can send the inputs around
// here, we just set bits manually, but you can find other ways to encode to bytes (for example by serializing)
#[allow(dead_code)]
pub fn input(_handle: In<PlayerHandle>, keyboard_input: Res<Input<KeyCode>>) -> Vec<u8> {
    let mut input: u8 = 0;

    if keyboard_input.pressed(KeyCode::W) {
        input |= INPUT_UP;
    }
    if keyboard_input.pressed(KeyCode::A) {
        input |= INPUT_LEFT;
    }
    if keyboard_input.pressed(KeyCode::S) {
        input |= INPUT_DOWN;
    }
    if keyboard_input.pressed(KeyCode::D) {
        input |= INPUT_RIGHT;
    }

    vec![input]
}
