use std::{collections::HashMap, fmt::Display};

use bevy::prelude::{App, Plugin, World};

use super::stats::WarframeStats;

pub enum AbilityName {
    // Excalibur
    SlashDash,
    RadialBlind,
    RadialJavelin,
    ExaltedBlade,

    // Volt
    Schock,
    Speed,
    ElectricShield,
    Discharge,

    // Mag
    Pull,
    Magnetize,
    Polarize,
    Crush,
}

pub trait Ability: Sync + Send {
    /// Description of the ability's effects.
    fn desc(&self) -> String;

    /// Formatting of all ability stats for display purposes.
    fn display(&self, wf_stats: &WarframeStats) -> HashMap<String, Box<dyn Display>>;

    /// Apply ability effects to the world.
    fn cast(&self, world: &mut World);
}

pub struct AbilityPlugin;

// impl Plugin for AbilityPlugin {
//     fn build(&self, app: &mut App) {
//         let mut abilities = HashMap::new();
//     }
// }
