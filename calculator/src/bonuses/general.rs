use crate::bonuses::{
    mul,
    perks::{CashPerk, XpPerk},
};

#[derive(Clone)]
pub struct GeneralMultipliers {
    pub group_boost: bool,

    pub cash_boost: bool,
    pub cash_gamepass: bool,
    pub cash_infamy_perks: CashPerk,

    pub xp_boost: bool,
    pub xp_gamepass: bool,
    pub xp_infamy_perks: XpPerk,
    pub xp_infamy_boost: bool,
}

impl GeneralMultipliers {
    pub fn get_cash_boost(self: &Self) -> f32 {
        mul(self.group_boost, 0.15)
            + mul(self.cash_boost, 1.0)
            + mul(self.cash_gamepass, 1.0)
            + self.cash_infamy_perks.get_bonus()
    }

    pub fn get_xp_boost(self: &Self) -> f32 {
        mul(self.group_boost, 0.15)
            + mul(self.xp_boost, 1.0)
            + mul(self.xp_gamepass, 1.0)
            + self.xp_infamy_perks.get_bonus()
            + mul(self.xp_infamy_boost, 0.15)
    }
}

impl Default for GeneralMultipliers {
    fn default() -> Self {
        GeneralMultipliers {
            group_boost: true,
            cash_boost: false,
            cash_gamepass: false,
            cash_infamy_perks: CashPerk::default(),
            xp_boost: false,
            xp_gamepass: false,
            xp_infamy_perks: XpPerk::default(),
            xp_infamy_boost: false,
        }
    }
}
