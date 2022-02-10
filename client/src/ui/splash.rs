use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::GameState;

fn splash_screen(mut egui_context: ResMut<EguiContext>) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
            |ui| ui.label(egui::RichText::new("DEATH'S EDGE").heading().strong()),
        )
    });
}

pub struct SplashUIPlugin;

impl Plugin for SplashUIPlugin {
    fn build(&self, app: &mut App) {
        let splash_set = SystemSet::on_update(GameState::Splash).with_system(splash_screen);
        app.add_system_set(splash_set);
    }
}
