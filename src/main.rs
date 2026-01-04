use bevy::{
    MinimalPlugins,
    app::{App, Update},
};
use bevy_ratatui::RatatuiPlugins;

use crate::bevy_based::{clock::clock_plugin, raindrop::raindrop_plugin, ui::ui_draw};

pub mod bevy_based;
pub mod config;
pub mod digits;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, RatatuiPlugins::default()))
        .add_plugins(raindrop_plugin)
        .add_plugins(clock_plugin)
        .add_systems(Update, ui_draw)
        .run();
}
