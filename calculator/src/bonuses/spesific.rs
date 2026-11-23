use crate::bonuses::{RepeatBonus, mul};

pub struct SpesificMultipliers {
    pub repeat_bonus: RepeatBonus,
    pub stealth_bonus: bool,
}

impl SpesificMultipliers {
    pub fn get_cash_boost(self: &Self) -> f32 {
        self.repeat_bonus.get_bonus()
    }
    pub fn get_xp_boost(self: &Self) -> f32 {
        self.repeat_bonus.get_bonus() + mul(self.stealth_bonus, 0.1)
    }
}
