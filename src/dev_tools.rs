use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((EguiPlugin::default(), WorldInspectorPlugin::new()));
}
