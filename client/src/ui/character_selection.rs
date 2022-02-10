use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use common::character::Class;

use crate::GameState;

fn character_selection(mut egui_context: ResMut<EguiContext>, mut selected_char: ResMut<Class>) {
    egui::TopBottomPanel::top("top").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            let classes = [
                Class::Mars,
                Class::Pluto,
                Class::Mammon,
                Class::Nergal,
                Class::Medea,
                Class::Janus,
                Class::Borvo,
                Class::Heka,
                Class::Rhea,
            ];
            for class in classes {
                ui.selectable_value(&mut *selected_char, class, class.as_str());
            }
        });
    });
}

pub struct CharacterSelectionPlugin;

impl Plugin for CharacterSelectionPlugin {
    fn build(&self, app: &mut App) {
        let set = SystemSet::on_update(GameState::MainLobby).with_system(character_selection);
        app.add_system_set(set);
    }
}
