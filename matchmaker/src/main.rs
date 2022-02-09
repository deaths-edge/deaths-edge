#![feature(int_abs_diff)]

use std::net::SocketAddr;

use bevy::prelude::*;
use common::{
    character::{Class, Role},
    network::{
        client::ClientMatchmakingMessage, find_my_ip_address, ClientAddress, NetworkEvent,
        NetworkResource, NetworkingPlugin,
    },
};

#[derive(Component)]
pub struct Rating(pub u32);

#[derive(Component)]
pub struct PlayerA(pub Entity);

#[derive(Component)]
pub struct PlayerB(pub Entity);

#[derive(Component)]
pub struct PlayerC(pub Entity);

#[derive(Component, Clone)]
pub struct Unpaired;

const PAIRING_DIFF: u32 = 50;
const MATCHING_DIFF: u32 = 100;
const SERVER_PORT: u16 = 8013;

// TODO: Switch to With?
fn pair_twos(players_query: Query<(Entity, &Class, &Rating, &Unpaired)>, mut commands: Commands) {
    for [(player_a, class_a, rating_a, _), (player_b, class_b, rating_b, _)] in
        players_query.iter_combinations::<2>()
    {
        // Can't pair two supports
        if class_a.role() == Role::Support && class_b.role() == Role::Support {
            continue;
        }

        let rating_diff = rating_a.0.abs_diff(rating_b.0);
        if rating_diff < PAIRING_DIFF {
            commands
                .spawn()
                .insert(PlayerA(player_a))
                .insert(PlayerB(player_b));
            commands.entity(player_a).remove::<Unpaired>();
            commands.entity(player_b).remove::<Unpaired>();
        }
    }
}

#[derive(Bundle)]
struct NewClient {
    address: ClientAddress,
    class: Class,
}

fn handle_client_messages(mut net: ResMut<NetworkResource>, mut commands: Commands) {
    for (connection_handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(client_message) = channels.recv::<ClientMatchmakingMessage>() {
            match client_message {
                ClientMatchmakingMessage::Enter(enter) => {
                    let new_client = NewClient {
                        address: ClientAddress(*connection_handle),
                        class: enter.class,
                    };
                    commands
                        .spawn()
                        .insert_bundle(new_client)
                        // TODO: Find rating
                        .insert(Rating(1_000));
                }
            }
        }
    }
}

fn matchmaking_twos(
    team_query: Query<(Entity, &PlayerA, &PlayerB)>,
    rating_query: Query<&Rating>,

    mut commands: Commands,
) {
    for [(blue_id, blue_player_a, blue_player_b), (red_id, red_player_a, red_player_b)] in
        team_query.iter_combinations::<2>()
    {
        let rating_blue = rating_query
            .get(blue_player_a.0)
            .expect("player not found")
            .0
            + rating_query
                .get(blue_player_b.0)
                .expect("player not found")
                .0;

        let rating_red = rating_query
            .get(red_player_a.0)
            .expect("player not found")
            .0
            + rating_query
                .get(red_player_b.0)
                .expect("player not found")
                .0;

        let rating_diff = rating_blue.abs_diff(rating_red);

        if rating_diff < MATCHING_DIFF {
            info!("matched");
            commands.entity(blue_id).despawn();
            commands.entity(red_id).despawn();
        }
    }
}

pub fn startup(mut net: ResMut<NetworkResource>) {
    let ip_address = find_my_ip_address().expect("can't find ip address");
    let address = SocketAddr::new(ip_address, SERVER_PORT);

    net.listen(address, None, None);
}

fn handle_connects(mut network_reader: EventReader<NetworkEvent>) {
    for event in network_reader.iter() {
        info!(message = "received network event", ?event);
        match event {
            NetworkEvent::Error(handle, error) => {
                error!(message = "timeout", %handle, ?error);
            }
            NetworkEvent::Connected(handle) => {
                info!(message = "connected", %handle);
            }
            NetworkEvent::Disconnected(handle) => {
                info!(message = "disconnected", %handle);
            }
            _ => (),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(NetworkingPlugin::default())
        .add_system(matchmaking_twos)
        .add_system(pair_twos)
        .add_system(handle_connects)
        .add_system(handle_client_messages)
        .run()
}
