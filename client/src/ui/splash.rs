use bevy::prelude::*;

use crate::state::ClientState;

use super::UIFonts;

#[derive(Debug, Component)]
pub struct SplashMarker;

#[derive(Bundle)]
pub struct SplashScreenRootBundle {
    marker: SplashMarker,
    #[bundle]
    node: NodeBundle,
}

impl SplashScreenRootBundle {
    pub fn new() -> Self {
        Self {
            marker: SplashMarker,
            node: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    ..Default::default()
                },
                color: Color::BLACK.into(),
                ..Default::default()
            },
        }
    }
}

pub fn setup_splash(mut commands: Commands, fonts: Res<UIFonts>) {
    commands
        .spawn_bundle(SplashScreenRootBundle::new())
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    ..Default::default()
                },
                text: Text::with_section(
                    "DEATH'S EDGE",
                    TextStyle {
                        color: Color::WHITE,
                        font_size: 150.0,
                        font: fonts.splash.clone(),
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            });
        });
}

pub fn remove_splash(mut commands: Commands, query: Query<Entity, With<SplashMarker>>) {
    let node = query.single();
    commands.entity(node).despawn_recursive();
}

pub struct SplashUIPlugin;

impl Plugin for SplashUIPlugin {
    fn build(&self, app: &mut App) {
        let teardown = SystemSet::on_exit(ClientState::Splash).with_system(remove_splash);
        app.add_startup_system(setup_splash)
            .add_system_set(teardown);
    }
}
