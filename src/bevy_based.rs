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

pub mod raindrop {
    use std::time::Duration;

    use bevy::{prelude::*, time::common_conditions::on_timer};
    use bevy_ratatui::RatatuiContext;
    use rand::Rng;

    pub fn raindrop_plugin(app: &mut App) {
        app
            // 雨滴落下动画
            .add_systems(
                Update,
                (rain_drop_fall, rain_drop_spawn)
                    .chain()
                    .run_if(on_timer(Duration::from_secs_f32(0.1))),
            );
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
    fn rain_drop_fall(
        q: Query<(&mut Position, Entity), With<Raindrop>>,
        ratatui: Res<RatatuiContext>,
        mut commands: Commands,
    ) {
        let size = ratatui.size().expect("io fail");
        for (mut position, entity) in q {
            let new_y = position.y + 1;
            if new_y >= size.height || position.x >= size.width {
                commands.entity(entity).despawn();
            }
            position.y = new_y;
        }
    }

    /// 新雨滴生成
    fn rain_drop_spawn(ratatui: Res<RatatuiContext>, mut commands: Commands) {
        let size = ratatui.size().expect("fail to get size");
        // XXX: bevy rand integration
        let mut rng = rand::rng();
        for x in 0..size.width {
            let n = rng.random_range(0..40);
            if n == 0 {
                commands.spawn((Raindrop, Position { x, y: 0 }));
            }
        }
    }
}

pub mod clock {

    use bevy::prelude::*;

    pub fn clock_plugin(app: &mut App) {
        app.init_resource::<Clock>()
            .add_systems(FixedUpdate, clock_sync);
    }

    /// We model time as a global resource.
    #[derive(Resource, Debug, Default)]
    pub struct Clock(pub chrono::NaiveTime);

    /// 时钟同步。
    ///
    /// 每秒钟让其与真实时钟同步。
    fn clock_sync(mut clock: ResMut<Clock>) {
        clock.0 = chrono::Local::now().time();
    }
}

// XXX: better bevy integration
/// 用户界面绘制
pub mod ui {
    use bevy::prelude::*;
    use bevy_ratatui::RatatuiContext;

    use crate::{
        bevy_based::{
            clock::Clock,
            raindrop::{Position, Raindrop},
        },
        digits::render_time_digits,
    };

    /// 雨滴绘制。
    pub fn ui_draw(
        mut context: ResMut<RatatuiContext>,
        q: Query<&Position, With<Raindrop>>,
        clock: Res<Clock>,
    ) {
        let size = context.size().expect("fail to get size");
        context
            .draw(|frame| {
                for position in q {
                    if position.x < size.width && position.y < size.height {
                        frame.render_widget(
                            "│",
                            ratatui::prelude::Rect {
                                x: position.x,
                                y: position.y,
                                width: 1,
                                height: 1,
                            },
                        );
                    }
                }

                render_time_digits(frame, clock.0, size.width, size.height);
            })
            .ok();
    }
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
