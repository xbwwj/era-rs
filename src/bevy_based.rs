use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_ratatui::RatatuiPlugins;

use crate::bevy_based::{clock::clock_plugin, raindrop::raindrop_plugin};

/// We model kind as a state.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum Kind {
    #[default]
    Clock,
    Counter,
}

// MARK: systems

mod raindrop {
    use bevy::prelude::*;
    use bevy_ratatui::RatatuiContext;

    pub fn raindrop_plugin(app: &mut App) {
        app
            // 雨滴落下动画
            .add_systems(FixedUpdate, (rain_drop_fall, rain_drop_spawn).chain())
            // 雨滴绘制
            .add_systems(Update, rain_drop_draw);
    }

    /// We model single raindrop as tag component.
    #[derive(Debug, Component)]
    pub struct Raindrop;

    /// The raindrop position.
    #[derive(Debug, Component)]
    pub struct Position {
        pub x: u16,
        pub y: u16,
    }

    /// 雨滴落下一格。
    ///
    /// 也要额外负责雨滴的 despawn
    fn rain_drop_fall() {
        todo!()
    }

    /// 新雨滴生成
    fn rain_drop_spawn() {
        todo!()
    }

    /// 雨滴绘制。
    fn rain_drop_draw(mut context: ResMut<RatatuiContext>) {
        todo!()
    }
}

mod clock {

    use bevy::prelude::*;

    pub fn clock_plugin(app: &mut App) {
        app.add_systems(FixedUpdate, clock_sync)
            .add_systems(Update, clock_draw);
    }

    /// We model time as a global resource.
    #[derive(Debug, Resource)]
    pub struct Clock(pub chrono::NaiveTime);

    /// 时钟同步。
    ///
    /// 每秒钟让其与真实时钟同步。
    // TODO: is there existing bevy chrono integration
    fn clock_sync() {}

    /// 时钟绘制。
    fn clock_draw() {}
}

pub fn bevy_main() {
    App::new()
        // 依赖插件
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(
                1. / 60.,
            ))),
            RatatuiPlugins::default(),
        ))
        // 逻辑定义
        .add_plugins((raindrop_plugin, clock_plugin))
        .run();
}
