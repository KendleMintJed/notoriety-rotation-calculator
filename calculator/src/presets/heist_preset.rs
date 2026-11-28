use crate::{Heist, bonuses::RepeatBonus, presets::StrategyPreset};

#[derive(Debug, Clone)]
pub struct HeistPreset {
    pub id: u32,
    pub name: String,
    pub stealth_only: bool,
}

impl HeistPreset {
    pub fn to_heist(
        self: Self,
        strategy_preset: StrategyPreset,
        repeat_bonus: RepeatBonus,
    ) -> Heist {
        Heist::new(self, strategy_preset, repeat_bonus)
    }
}
