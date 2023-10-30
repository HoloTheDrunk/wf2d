use std::sync::Arc;

use bevy::prelude::*;

type Effect = dyn Fn(&mut World) + Send + Sync;

#[derive(Clone, Component)]
struct Mod {
    effect: Arc<Effect>,
}

type ModSlot = Option<Mod>;

#[derive(Component)]
struct ModConfig<const MOD_SLOTS: usize = 4, const ARCANE_SLOTS: usize = 0> {
    special: ModSlot,
    mods: [ModSlot; MOD_SLOTS],
    arcanes: [ModSlot; ARCANE_SLOTS],
}

type RangedConfig = ModConfig<4, 1>;

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
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, init_mods)
        .add_systems(PostStartup, (manual_query, auto_query))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
