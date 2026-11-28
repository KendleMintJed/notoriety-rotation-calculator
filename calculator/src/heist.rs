use crate::{
    Rewards, Strategy,
    bonuses::{GeneralMultipliers, RepeatBonus, SpesificMultipliers},
    heist,
    presets::{HeistPreset, StrategyPreset},
};

#[derive(Debug, Clone)]
pub struct Heist {
    heist_preset: HeistPreset,
    strategy_preset: StrategyPreset,
    multipliers: SpesificMultipliers,
}

impl Heist {
    pub fn new(
        heist_preset: HeistPreset,
        strategy_preset: StrategyPreset,
        repeat_bonus: RepeatBonus,
    ) -> Self {
        let stealth = strategy_preset.strategy.stealth;
        let stealth_only = heist_preset.stealth_only;
        Self {
            heist_preset,
            strategy_preset,
            multipliers: SpesificMultipliers {
                repeat_bonus,
                stealth_bonus: stealth && !stealth_only,
            },
        }
    }

    pub fn get_reward(self: &Self, general: &GeneralMultipliers) -> Rewards {
        let spesific = &self.multipliers;
        let strategy = &self.strategy_preset.strategy;

        let cash_multiplier = 1.0 + general.get_cash_boost() + spesific.get_cash_boost();

        let xp_multiplier =
            1.0 + general.get_xp_boost() + spesific.get_xp_boost() + strategy.crew.get_bonus();

        let cash = (strategy.base_cash as f32 * cash_multiplier) as u32;
        let xp = (strategy.base_xp as f32 * xp_multiplier) as u32;

        Rewards { cash, xp }
    }

    pub fn get_heist_preset(self: &Self) -> &HeistPreset {
        &self.heist_preset
    }
    pub fn get_strategy_preset(self: &Self) -> &StrategyPreset {
        &self.strategy_preset
    }
    pub fn get_multipliers(self: &Self) -> &SpesificMultipliers {
        &self.multipliers
    }
    pub fn get_time(self: &Self) -> f32 {
        self.strategy_preset.strategy.in_game_time
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            bonuses::{
                Crew, RepeatBonus,
                perks::{CashPerk, SkillPerk, XpPerk},
            },
            presets::PresetStore,
        },
    };
    #[test]
    fn test_nc_fs_generation() {
        let general_multipliers = GeneralMultipliers {
            group_boost: true,

            cash_boost: true,
            cash_gamepass: true,
            cash_infamy_perks: CashPerk::new(8).unwrap(),

            xp_boost: true,
            xp_gamepass: true,
            xp_infamy_perks: XpPerk::new(8).unwrap(),
            xp_infamy_boost: false,

            skill_perks: SkillPerk::default(),
        };
        let spesific_multipliers = SpesificMultipliers {
            repeat_bonus: RepeatBonus::new(0).unwrap(),
            stealth_bonus: true,
        };

        let preset_store = PresetStore::generate_default_presets();
        let preset_store = preset_store.get_data();

        let heist = Heist {
            heist_preset: preset_store[0].0.clone(),
            strategy_preset: preset_store[0].1[0].clone(),
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
