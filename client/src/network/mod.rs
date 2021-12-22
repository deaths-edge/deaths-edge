mod character_command;

use std::{marker::PhantomData, net::SocketAddr};

use bevy::prelude::*;

use common::{
    character::{Action, CharacterClass, CharacterTeam, FocalAngle, Motion, CHARACTER_COMMANDS},
    game::{ArenaPasscode, ArenaPermit},
    network::{
        client::ClientMessage,
        network_setup,
        server::{CharacterCommand, GameCommand, Reconcile, ServerMessage, SpawnCharacter},
        CharacterNetworkCommand, NetworkEvent, NetworkResource, NetworkingPlugin,
        NETWORK_SETUP_LABEL,
    },
};

use crate::{
    character::PlayerState,
    input_mapping::{PlayerInputCommand, INPUT_TO_CHARACTER_LABEL},
    spawning::SPAWN_CHARACTER_LABEL,
    state::ClientState,
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

fn request_arena_entry(mut net: ResMut<NetworkResource>, opts: Res<Opt>) {
    info!("sending arena permit");
    let message = ClientMessage::Permit(ArenaPermit::new(
        ArenaPasscode(opts.passcode),
        CharacterClass::Medea,
        CharacterTeam::Red,
    ));
    net.broadcast_message(message);
}

pub fn handle_server_messages(
    mut net: ResMut<NetworkResource>,
    mut spawn_writer: EventWriter<SpawnCharacter>,
    mut motion_writer: EventWriter<CharacterNetworkCommand<Motion>>,
    mut action_writer: EventWriter<CharacterNetworkCommand<Action>>,
    mut focal_angle_writer: EventWriter<CharacterNetworkCommand<FocalAngle>>,
    mut reconcile_writer: EventWriter<Reconcile>,
) {
    for (_, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();

        while let Some(server_message) = channels.recv::<ServerMessage>() {
            match server_message {
                ServerMessage::ArenaPasscodeAck => {}
                ServerMessage::GameCommand(command) => match command {
                    GameCommand::SpawnCharacter(spawn) => spawn_writer.send(spawn),
                },
                ServerMessage::CharacterCommand(command) => match command {
                    CharacterCommand::Motion(motion) => motion_writer.send(motion),
                    CharacterCommand::Action(action) => action_writer.send(action),
                    CharacterCommand::FocalAngle(angle) => focal_angle_writer.send(angle),
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

/// Listens to [`PlayerInputCommand`] and sends the internal value to the server.
fn player_input_to_network<Value>(
    mut input_commands: EventReader<PlayerInputCommand<Value>>,
    mut net: ResMut<NetworkResource>,
) where
    Value: Clone + Send + Sync + 'static,
    Value: Into<ClientMessage>,
{
    for command in input_commands.iter().cloned() {
        let command: ClientMessage = command.0.into();
        net.broadcast_message(command);
    }
}

pub struct NetworkPlugin;

pub const CHARACTER_NETWORK_COMMAND_LABEL: &str = "broadcast-inputs";

pub struct CharacterNetworkCommandPlugin<T> {
    _command: PhantomData<T>,
}

impl<T> CharacterNetworkCommandPlugin<T> {
    pub fn new() -> Self {
        Self {
            _command: PhantomData,
        }
    }
}

pub fn startup(mut net: ResMut<NetworkResource>, game_server: Res<GameServer>) {
    net.connect(game_server.address());
}

pub const NETWORK_TO_ENTITY_LABEL: &str = "network-to-entity";

impl<T> Plugin for CharacterNetworkCommandPlugin<T>
where
    T: Send + Sync + 'static,
    T: Clone,
    T: Into<ClientMessage>,
{
    fn build(&self, app: &mut AppBuilder) {
        let broadcast_inputs = SystemSet::on_update(PlayerState::Spawned)
            .label(CHARACTER_NETWORK_COMMAND_LABEL)
            // INPUT_TO_CHARACTER_LABEL sends PlayerInputCommand<Value> events
            .after(INPUT_TO_CHARACTER_LABEL)
            .with_system(player_input_to_network::<T>.system());

        let network_to_entity = SystemSet::on_update(ClientState::Arena)
            .label(NETWORK_TO_ENTITY_LABEL)
            // NETWORK_HANDLE_LABEL sends CharacterNetworkCommand<Value> events
            .after(NETWORK_HANDLE_LABEL)
            // CHARACTER_COMMANDS reads CharacterEntityCommand<Value> events
            .before(CHARACTER_COMMANDS)
            .with_system(network_to_entity_command::<T>.system());

        app.add_event::<CharacterNetworkCommand<T>>()
            .add_system_set(broadcast_inputs)
            .add_system_set(network_to_entity);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum NetworkConnectivity {
    Connected,
    Disconnected,
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let setup = SystemSet::on_enter(ClientState::Arena)
            .label(NETWORK_SETUP_LABEL)
            .with_system(startup.system())
            .with_system(network_setup.system());

        // Request entry to arena
        // TODO: Do this in lobby
        let send_passcode = SystemSet::on_enter(NetworkConnectivity::Connected)
            .with_system(request_arena_entry.system());

        let handle_server_message = SystemSet::on_update(ClientState::Arena)
            .label(NETWORK_HANDLE_LABEL)
            // NETWORK_TO_ENTITY_LABEL reads CharacterNetworkCommand<Value> events
            .before(NETWORK_TO_ENTITY_LABEL)
            // SPAWN_CHARACTER_LABEL reads SpawnCharacter events
            .before(SPAWN_CHARACTER_LABEL)
            .with_system(handle_server_messages.system())
            .with_system(handle_connects.system());

        app.add_state(NetworkConnectivity::Disconnected)
            .add_plugin(NetworkingPlugin::default())
            .add_plugin(CharacterNetworkCommandPlugin::<Motion>::new())
            .add_plugin(CharacterNetworkCommandPlugin::<Action>::new())
            .add_plugin(CharacterNetworkCommandPlugin::<FocalAngle>::new())
            .add_system_set(setup)
            .add_system_set(send_passcode)
            .add_system_set(handle_server_message);
    }
}
