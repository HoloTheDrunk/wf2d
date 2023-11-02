mod model;

use model::mods::*;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};

fn init_mods(world: &mut World) {
    world.query::<&RangedConfig>().iter(world);
}

fn print_query(query: Query<&RangedConfig>) {
    query
        .par_iter()
        .for_each(|config| println!("print: {config:?}"))
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()))
        .add_systems(PreStartup, init_mods)
        .add_systems(PostStartup, print_query)
        .run();
}
