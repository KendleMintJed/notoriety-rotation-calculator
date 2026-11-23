use crate::{
    Heist, Rewards,
    bonuses::{GeneralMultipliers, RepeatBonus},
    presets::{HeistPreset, StrategyPreset},
};

#[derive(Clone, Default)]
pub struct Rotation(pub Vec<(HeistPreset, StrategyPreset)>);

impl Rotation {
    pub fn to_repeats(
        self: &Self,
    ) -> impl Iterator<Item = (HeistPreset, StrategyPreset, RepeatBonus)> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, (heist_preset, strategy_preset))| {
                let mut repeat = 0;
                for j in 1..=7 {
                    if heist_preset.id
                        == self.0[(i as i32 - j).wrapping_rem_euclid(self.0.len() as i32) as usize]
                            .0
                            .id
                    {
                        repeat += 1;
                    }
                }

                // Never out of range
                (
                    heist_preset.clone(),
                    strategy_preset.clone(),
                    RepeatBonus::new(repeat).unwrap(),
                )
            })
    }

    pub fn to_heists(self: &Self) -> impl Iterator<Item = Heist> {
        self.to_repeats()
            .map(|(heist_preset, strategy_preset, repeat_bonus)| {
                heist_preset.to_heist(strategy_preset, repeat_bonus)
            })
    }

    pub fn to_rewards(
        self: &Self,
        multipliers: &GeneralMultipliers,
    ) -> impl Iterator<Item = Rewards> {
        self.to_heists().map(|heist| heist.get_reward(multipliers))
    }
}
