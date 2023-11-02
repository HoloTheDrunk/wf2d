use super::{ability::AbilityName, mods::WarframeConfig, stats::WarframeStats};

use std::{collections::HashMap, fmt::Display, sync::Arc};

use bevy::prelude::{Bundle, Component, World};

// pub trait Ability: Sync + Send {
//     /// Description of the ability's effects.
//     fn desc(&self) -> String;
//
//     /// Formatting of all ability stats for display purposes.
//     fn display(&self, wf_stats: &WarframeStats) -> HashMap<String, Box<dyn Display>>;
//
//     /// Apply ability effects to the world.
//     fn cast(&self, world: &mut World);
// }

#[derive(Component, Debug, strum::Display)]
enum Warframe {
    Excalibur,
    Rhino,
    Mag,
}

#[derive(Component)]
pub struct Abilities {
    abilities: [AbilityName; 4],
}

#[derive(Bundle)]
pub struct WarframeBundle {
    warframe: Warframe,
    abilities: Abilities,
    config: WarframeConfig,
    stats: WarframeStats,
}
