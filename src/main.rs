use std::sync::Arc;

use bevy::prelude::*;

type Effect = dyn Fn(&mut World) + Send + Sync;

#[derive(Component)]
struct Mod {
    effect: Arc<Effect>,
}

fn init_mods(mut commands: Commands) {
    commands.spawn(Mod {
        effect: Arc::new(|_| println!("Called!")),
    });
}

fn manual_query(world: &mut World) {
    for (i, _mod) in world.query::<&Mod>().iter(world).enumerate() {
        println!("manual: {i}");
    }
}

fn auto_query(query: Query<&Mod>) {
    for (i, _mod) in query.iter().enumerate() {
        println!("auto: {i}");
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init_mods)
        .add_systems(PostStartup, (manual_query, auto_query))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
