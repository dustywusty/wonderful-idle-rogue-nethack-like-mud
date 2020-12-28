use bevy::{
    app::{ScheduleRunnerPlugin, ScheduleRunnerSettings},
    ecs::prelude::*,
    prelude::*,
    utils::Duration,
};
use bevy_prototype_simple_net::{ListenerId, NetPlugin};

// ================================================================================================
// ================================================================================================

mod components;
use components::*;

mod systems;
use systems::*;

// ================================================================================================
// ================================================================================================

fn main() {
    App::build()
        // Uncomment this to override the default log settings:
        // .add_resource(bevy::log::LogSettings {
        //     level: bevy::log::Level::TRACE,
        //     filter: "wgpu=warn,bevy_ecs=info".to_string(),
        // })
        .add_plugins(DefaultPlugins)
        .add_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(500)))
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(NetPlugin)
        .add_resource(ListenerId::new())
        .add_startup_system(setup_network_listener.system())
        .add_system_to_stage(stage::UPDATE, accept_connections_system.system())
        .add_system_to_stage(stage::UPDATE, handle_incoming_data.system())
        .add_system_to_stage(stage::UPDATE, handle_error_system.system())
        .run();
}
