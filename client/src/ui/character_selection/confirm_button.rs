use std::net::SocketAddr;

use bevy::prelude::*;
use common::network::{find_my_ip_address, SERVER_PORT};

use crate::state::StateTransition;

use super::CharacterSelectionMaterials;

pub struct CharacterConfirmButtonMarker;

#[derive(Bundle)]
pub struct CharacterConfirmButton {
    marker: CharacterConfirmButtonMarker,
    #[bundle]
    button: ButtonBundle,
}

impl CharacterConfirmButton {
    pub fn new(materials: &CharacterSelectionMaterials) -> Self {
        Self {
            marker: CharacterConfirmButtonMarker,
            button: ButtonBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    size: Size::new(Val::Px(65.0), Val::Px(35.0)),
                    // margin: Rect::all(Val::Auto),
                    // justify_content: JustifyContent::Center,
                    // align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: materials.confirm_button.clone(),
                ..Default::default()
            },
        }
    }
}

pub fn handle_confirm_click(
    mut interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<Button>,
            With<CharacterConfirmButtonMarker>,
        ),
    >,
    mut state_transition: EventWriter<StateTransition>,
) {
    for interaction in interaction_query.iter_mut() {
        let ip_address = find_my_ip_address().expect("can't find ip address");
        let server = SocketAddr::new(ip_address, SERVER_PORT);

        match *interaction {
            Interaction::Clicked => state_transition.send(StateTransition::Connect { server }),
            _ => (),
        }
    }
}
