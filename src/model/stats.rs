use std::{
    collections::HashMap,
    fmt::Display,
    ops::{AddAssign, MulAssign},
    time::{Duration, Instant},
};

use bevy::prelude::Component;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Trigger {
    OnHit,
    OnCritHit,
    OnStatusEffectDealt(StatusEffect),
    OnStatusEffectTaken(StatusEffect),
    OnEnergyPickup,
    OnHealthPickup,
    OnAbilityUse,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StatusEffect {
    Slash,
    Puncture,
    Impact,
}

#[derive(PartialEq, Eq, Hash, Debug, strum::Display)]
pub enum ModName {
    Serration,
    EnergyConversion,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StatModifierType {
    Additive,
    Multiplicative,
}

#[derive(Clone, Debug)]
pub enum StatModifierDuration {
    Timed { end: Instant, is_refreshable: bool },
    Uses { trigger: Trigger, count: u32 },
    Unlimited,
}

impl StatModifierDuration {
    fn new_timed(duration: Duration, is_refreshable: bool) -> Self {
        Self::Timed {
            end: Instant::now() + duration,
            is_refreshable,
        }
    }
}

#[derive(Debug)]
pub struct StatModifier {
    amount: f32,
    r#type: StatModifierType,
    duration: StatModifierDuration,
}

impl StatModifier {
    fn update_to(&mut self, modifier: &StatModifier) {
        self.amount = self.amount.max(modifier.amount);
        self.duration = modifier.duration.clone();
    }
}

#[derive(Default, Debug)]
pub struct Stat<TYPE> {
    base: TYPE,
    modifiers: HashMap<(ModName, u32), StatModifier>,
}

impl<TYPE: Copy + AddAssign + MulAssign + From<f32>> Stat<TYPE> {
    fn add_modifier(&mut self, variant: (ModName, u32), modifier: StatModifier) {
        self.modifiers
            .entry(variant)
            .and_modify(|modi| modi.update_to(&modifier))
            .or_insert(modifier);
    }

    fn get_modifier(&self, variant: (ModName, u32)) -> Option<&StatModifier> {
        self.modifiers.get(&variant)
    }

    fn final_value(&self) -> TYPE {
        let mut res = self.base;

        for modifier in self
            .modifiers
            .values()
            .filter(|modi| modi.r#type == StatModifierType::Additive)
        {
            res += modifier.amount.into();
        }

        for modifier in self
            .modifiers
            .values()
            .filter(|modi| modi.r#type == StatModifierType::Multiplicative)
        {
            res *= modifier.amount.into();
        }

        res
    }
}

impl<TYPE: Display + Copy + AddAssign + MulAssign + From<f32>> Display for Stat<TYPE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.final_value().to_string().as_ref())
    }
}

pub struct LifeStats {
    pub shield: Stat<u32>,
    pub armor: Stat<u32>,
    pub health: Stat<u32>,
}

pub struct AbilityStats {
    pub strength: Stat<f32>,
    pub duration: Stat<f32>,
    pub range: Stat<f32>,
    pub efficiency: Stat<f32>,
}

#[derive(Component)]
pub struct WarframeStats {
    pub energy: Stat<u32>,
    pub life: LifeStats,
    pub ability: AbilityStats,
}
