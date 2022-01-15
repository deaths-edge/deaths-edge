mod confirm_button;
mod cycle_button;

use bevy::prelude::*;
use common::character::Class;

use crate::state::ClientState;

use confirm_button::*;
use cycle_button::*;

use super::UIFonts;

#[derive(Default, Debug, Component)]
struct CharacterSelectionRootMarker;

#[derive(Bundle)]
pub struct CharacterSelectionRoot {
    marker: CharacterSelectionRootMarker,
    #[bundle]
    node: NodeBundle,
}

impl CharacterSelectionRoot {
    pub fn new() -> Self {
        Self {
            marker: CharacterSelectionRootMarker,
            node: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                color: Color::BLACK.into(),
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Default, Component)]
struct CharacterSelectionTextMarker;

#[derive(Bundle)]
pub struct ClassSelectionText {
    marker: CharacterSelectionTextMarker,

    #[bundle]
    text: TextBundle,
}

impl ClassSelectionText {
    pub fn new(character_class: Class, fonts: &UIFonts) -> Self {
        Self {
            marker: CharacterSelectionTextMarker,
            text: TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    ..Default::default()
                },
                text: Text::with_section(
                    character_class.as_str(),
                    TextStyle {
                        color: Color::WHITE,
                        font_size: 70.0,
                        font: fonts.character_selection.clone(),
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            },
        }
    }
}

pub struct CharacterSelectionPlugin;

pub fn spawn_character_selection(
    selected_char: Res<Class>,
    fonts: Res<UIFonts>,

    mut commands: Commands,
) {
    commands
        .spawn_bundle(CharacterSelectionRoot::new())
        .with_children(|parent| {
            parent.spawn_bundle(CharacterConfirmButton::new());

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
                        position_type: PositionType::Relative,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(CharacterCycleButton::new_left());
                    parent.spawn_bundle(ClassSelectionText::new(*selected_char, &fonts));
                    parent.spawn_bundle(CharacterCycleButton::new_right());
                });
        });
}

fn despawn_character_selection(
    query: Query<Entity, With<CharacterSelectionRootMarker>>,
    mut commands: Commands,
) {
    let id = query.single();
    commands.entity(id).despawn_recursive();
}

fn character_text_changed(
    selected_char: Res<Class>,
    mut text_query: Query<&mut Text, With<CharacterSelectionTextMarker>>,
) {
    if selected_char.is_changed() {
        let mut text = text_query.single_mut();
        text.sections[0].value = selected_char.to_string();
    }
}

impl Plugin for CharacterSelectionPlugin {
    fn build(&self, app: &mut App) {
        const HANDLE_SELECTION_CLICKS: &str = "handle-selection-clicks";
        let spawn_ui = SystemSet::on_enter(ClientState::MainLobby)
            .with_system(spawn_character_selection.system());

        let despawn_ui = SystemSet::on_exit(ClientState::MainLobby)
            .with_system(despawn_character_selection.system());

        let handle_clicks = SystemSet::on_update(ClientState::MainLobby)
            .label(HANDLE_SELECTION_CLICKS)
            .with_system(handle_click_left.system())
            .with_system(handle_click_right.system())
            .with_system(handle_confirm_click.system());

        let character_class = SystemSet::on_update(ClientState::MainLobby)
            .with_system(character_text_changed.system());

        app.insert_resource(Class::Heka)
            .add_system_set(spawn_ui)
            .add_system_set(despawn_ui)
            .add_system_set(handle_clicks)
            .add_system_set(character_class);
    }
}
