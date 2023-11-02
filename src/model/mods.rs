use super::equipment::*;

use std::{fmt::Debug, sync::Arc};

use bevy::prelude::{Component, World};

#[derive(Debug, Clone)]
pub enum Polarity {
    Naramon,
    Vazarin,
    Madurai,
    Zenurik,
    Unairu,
    Umbra,
    // Companion precepts
    Penjaga,
    // Aura and posture universal polarity
    Special,
}

pub type Effect = dyn Fn(&mut World) + Send + Sync;

#[derive(Clone)]
pub struct Mod {
    pub cost: u8,
    pub polarity: Polarity,
    pub eqtype: EqType,
    pub effect: Arc<Effect>,
}

impl Debug for Mod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            cost,
            polarity,
            eqtype,
            effect: _,
        } = self;

        f.debug_struct(stringify!(Mod))
            .field(stringify!(cost), &cost.to_string())
            .field(stringify!(polarity), &format!("{polarity:?}"))
            .field(stringify!(eqtype), &format!("{eqtype:?}"))
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Default)]
pub struct ModSlot {
    pub polarity: Option<Polarity>,
    pub inner: Option<Mod>,
}

#[derive(Debug, Component)]
pub struct ModConfig<const MOD_SLOTS: usize, const SPECIAL_SLOTS: usize, const ARCANE_SLOTS: usize>
{
    pub mods: [ModSlot; MOD_SLOTS],
    pub special: [ModSlot; SPECIAL_SLOTS],
    pub arcanes: [ModSlot; ARCANE_SLOTS],
}

impl<const MOD_SLOTS: usize, const SPECIAL_SLOTS: usize, const ARCANE_SLOTS: usize> Default
    for ModConfig<MOD_SLOTS, SPECIAL_SLOTS, ARCANE_SLOTS>
{
    fn default() -> Self {
        Self {
            mods: std::array::from_fn(|_| ModSlot::default()),
            special: std::array::from_fn(|_| ModSlot::default()),
            arcanes: std::array::from_fn(|_| ModSlot::default()),
        }
    }
}

pub type WarframeConfig = ModConfig<4, 1, 2>;
pub type RangedConfig = ModConfig<4, 1, 1>;
pub type MeleeConfig = ModConfig<4, 1, 1>;
pub type CompanionConfig = ModConfig<4, 0, 0>;
