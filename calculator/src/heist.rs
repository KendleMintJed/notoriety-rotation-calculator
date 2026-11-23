use crate::{
    Rewards, Strategy,
    bonuses::{GeneralMultipliers, RepeatBonus, SpesificMultipliers},
};

pub struct Heist {
    strategy: Strategy,
    multipliers: SpesificMultipliers,
}

impl Heist {
    pub fn new(strategy: Strategy, stealth_only: bool, repeat_bonus: RepeatBonus) -> Heist {
        let stealth = strategy.stealth;
        Heist {
            strategy,
            multipliers: SpesificMultipliers {
                repeat_bonus,
                stealth_bonus: stealth && !stealth_only,
            },
        }
    }
    pub fn get_reward(self: &Self, general: &GeneralMultipliers) -> Rewards {
        let spesific = &self.multipliers;
        let strategy = &self.strategy;

        let cash_multiplier = 1.0 + general.get_cash_boost() + spesific.get_cash_boost();

        let xp_multiplier =
            1.0 + general.get_xp_boost() + spesific.get_xp_boost() + strategy.crew.get_bonus();

        let cash = (strategy.base_cash as f32 * cash_multiplier) as u32;
        let xp = (strategy.base_xp as f32 * xp_multiplier) as u32;

        Rewards { cash, xp }
    }
    pub fn get_strategy(self: &Self) -> &Strategy {
        &self.strategy
    }
    pub fn get_multipliers(self: &Self) -> &SpesificMultipliers {
        &self.multipliers
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::bonuses::{
            Crew, RepeatBonus,
            perks::{CashPerk, XpPerk},
        },
    };
    #[test]
    fn test_nc_fs_generation() {
        let strategy = Strategy {
            base_xp: 840_000,
            base_cash: 1_079_550,
            in_game_time: 24.0,
            stealth: true,
            crew: Crew::new(4).unwrap(),
        };
        let general_multipliers = GeneralMultipliers {
            group_boost: true,

            cash_boost: true,
            cash_gamepass: true,
            cash_infamy_perks: CashPerk::new(8).unwrap(),

            xp_boost: true,
            xp_gamepass: true,
            xp_infamy_perks: XpPerk::new(8).unwrap(),
            xp_infamy_boost: false,
        };
        let spesific_multipliers = SpesificMultipliers {
            repeat_bonus: RepeatBonus::new(0).unwrap(),
            stealth_bonus: true,
        };
        let heist = Heist {
            strategy,
            multipliers: spesific_multipliers,
        };

        assert_eq!(
            heist.get_reward(&general_multipliers),
            Rewards {
                cash: 5_343_772,
                xp: 4_200_000,
            }
        )
    }
}
