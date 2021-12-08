use std::collections::VecDeque;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[derive(Default)]
pub struct FPSHistory {
    history: VecDeque<(&'static str, u64)>,
}

impl FPSHistory {
    pub fn history(&self) -> &[(&'static str, u64)] {
        self.history.as_slices().0
    }
}

pub fn collect_fps(diagnostics: Res<Diagnostics>, mut fps_history: ResMut<FPSHistory>) {
    if let Some(diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = diagnostic.value() {
            if fps_history.history.len() > 26 {
                fps_history.history.pop_front();
            }
            fps_history.history.push_back(("", value as u64));
            fps_history.history.make_contiguous();
        }
    }
}
