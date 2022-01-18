mod character_command;

use std::net::SocketAddr;

use bevy::prelude::*;

use common::{
    character::{Ability, Class, FocalAngle, Motion, SelectTarget, Team},
    game::{ArenaPasscode, ArenaPermit},
    network::{
        client::ClientMessage,
        network_setup,
        server::{CharacterAction, GameAction, Reconcile, ServerMessage, SpawnCharacter},
        CharacterNetworkAction, NetworkEvent, NetworkResource, NetworkingPlugin,
        NETWORK_SETUP_LABEL,
    },
};

use crate::{
    input_mapping::PlayerInputAction,
    spawning::SPAWN_CHARACTER_LABEL,
    state::{ClientState, StateTransition},
    Opt,
};

use character_command::*;

pub const NETWORK_HANDLE_LABEL: &str = "network-handle";

pub struct GameServer {
    address: SocketAddr,
}

impl GameServer {
    pub fn new(address: SocketAddr) -> Self {
        Self { address }
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }
}

fn request_arena_entry(mut net: ResMut<NetworkResource>, class: Res<Class>, opts: Res<Opt>) {
    info!("sending arena permit");

    // TODO: Don't do this
    let team = match *class {
        Class::Mars => Team::Red,
        _ => Team::Blue,
    };

    let message = ClientMessage::Permit(ArenaPermit {
        passcode: ArenaPasscode(opts.passcode),
        class: *class,
        team,
    });
    net.broadcast_message(message);
}

pub fn handle_server_messages(
    mut net: ResMut<NetworkResource>,

    mut spawn_writer: EventWriter<SpawnCharacter>,

    mut motion_writer: EventWriter<CharacterNetworkAction<Motion>>,
    mut target_writer: EventWriter<CharacterNetworkAction<SelectTarget>>,
    mut ability_writer: EventWriter<CharacterNetworkAction<Ability>>,
    mut focal_angle_writer: EventWriter<CharacterNetworkAction<FocalAngle>>,

    mut reconcile_writer: EventWriter<Reconcile>,

    mut transition: EventWriter<StateTransition>,
) {
    for (_, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();

        while let Some(server_message) = channels.recv::<ServerMessage>() {
            match server_message {
                ServerMessage::GameAction(action) => match action {
                    GameAction::SpawnCharacter(spawn) => spawn_writer.send(spawn),
                    GameAction::Setup(setup) => {
                        transition.send(StateTransition::Connected { setup })
                    }
                },
                ServerMessage::CharacterAction(action) => match action {
                    CharacterAction::Motion(motion) => motion_writer.send(motion),
                    CharacterAction::OptionalTarget(target) => target_writer.send(target),
                    CharacterAction::Ability(ability) => ability_writer.send(ability),
                    CharacterAction::FocalAngle(angle) => focal_angle_writer.send(angle),
                },
                ServerMessage::Reconcile(reconcile) => reconcile_writer.send(reconcile),
            }
        }
    }
}

fn handle_connects(
    mut network_reader: EventReader<NetworkEvent>,
    mut connectivity: ResMut<State<NetworkConnectivity>>,
) {
    for event in network_reader.iter() {
        match event {
            NetworkEvent::Error(handle, error) => {
                error!(message = "error", %handle, ?error);
            }
            NetworkEvent::Connected(handle) => {
                info!(message = "connected", %handle);
                connectivity
                    .set(NetworkConnectivity::Connected)
                    .expect("already connected");
            }
            NetworkEvent::Disconnected(handle) => {
                info!(message = "disconnect", %handle);
            }
            _ => (),
        }
    }
}

/// Listens to [`PlayerInputAction`] and sends the internal value to the server.
fn player_input_to_network<Value>(
    mut input_commands: EventReader<PlayerInputAction<Value>>,
    mut net: ResMut<NetworkResource>,
) where
    Value: Clone + Send + Sync + 'static,
    Value: Into<ClientMessage>,
{
    for action in input_commands.iter().cloned() {
        let action: ClientMessage = action.0.into();
        net.broadcast_message(action);
    }
}

pub struct NetworkPlugin;

pub fn startup(mut net: ResMut<NetworkResource>, game_server: Res<GameServer>) {
    net.connect(game_server.address());
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum NetworkConnectivity {
    Connected,
    Disconnected,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum NetworkingState {
    Active,
    Sleep,
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        let setup = SystemSet::on_enter(ClientState::Connecting)
            .label(NETWORK_SETUP_LABEL)
            .with_system(startup)
            .with_system(network_setup);

        // Request entry to arena
        // TODO: Do this in lobby
        let send_passcode =
            SystemSet::on_enter(NetworkConnectivity::Connected).with_system(request_arena_entry);

        let handle_server_message = SystemSet::on_update(NetworkingState::Active)
            .label(NETWORK_HANDLE_LABEL)
            // NETWORK_TO_ENTITY_LABEL reads CharacterNetworkAction<Value> events
            .before(NETWORK_TO_ENTITY_LABEL)
            // SPAWN_CHARACTER_LABEL reads SpawnCharacter events
            .before(SPAWN_CHARACTER_LABEL)
            .with_system(handle_server_messages)
            .with_system(handle_connects);

        app.add_state(NetworkConnectivity::Disconnected)
            .add_state(NetworkingState::Sleep)
            .add_plugin(NetworkingPlugin::default())
            .add_plugin(CharacterNetworkActionPlugin::<Motion>::new())
            .add_plugin(CharacterNetworkActionPlugin::<SelectTarget>::new())
            .add_plugin(CharacterNetworkActionPlugin::<Ability>::new())
            .add_plugin(CharacterNetworkActionPlugin::<FocalAngle>::new())
            .add_system_set(setup)
            .add_system_set(send_passcode)
            .add_system_set(handle_server_message);
    }
}
