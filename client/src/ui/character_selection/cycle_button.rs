use bevy::prelude::*;
use common::character::Class;

#[derive(Default, Debug, Component)]
pub struct CharacterCycleButtonMarkerLeft;

#[derive(Default, Debug, Component)]
pub struct CharacterCycleButtonMarkerRight;

#[derive(Bundle)]
pub struct CharacterCycleButton<T>
where
    T: Component + Send + Sync + 'static,
{
    marker: T,
    #[bundle]
    button: ButtonBundle,
}

impl<T> CharacterCycleButton<T>
where
    T: Component,
{
    pub fn new(marker: T) -> Self {
        Self {
            marker,
            button: ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(65.0), Val::Px(35.0)),
                    // margin: Rect::all(Val::Auto),
                    // justify_content: JustifyContent::Center,
                    // align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: Color::CRIMSON.into(),
                ..Default::default()
            },
        }
    }
}

impl CharacterCycleButton<CharacterCycleButtonMarkerLeft> {
    pub fn new_left() -> Self {
        Self::new(CharacterCycleButtonMarkerLeft)
    }
}

impl CharacterCycleButton<CharacterCycleButtonMarkerRight> {
    pub fn new_right() -> Self {
        Self::new(CharacterCycleButtonMarkerRight)
    }
}

pub fn handle_click_left(
    mut interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<Button>,
            With<CharacterCycleButtonMarkerLeft>,
        ),
    >,
    mut selected_char: ResMut<Class>,
) {
    for interaction in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *selected_char = selected_char.cycle_left();
            }
            _ => (),
        }
    }
}

pub fn handle_click_right(
    mut interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<Button>,
            With<CharacterCycleButtonMarkerRight>,
        ),
    >,
    mut selected_char: ResMut<Class>,
) {
    for interaction in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *selected_char = selected_char.cycle_right();
            }
            _ => (),
        }
    }
}
