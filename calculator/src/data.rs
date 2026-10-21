use crate::{Crew, GeneralMultipliers, Heist, RepeatBonus, Rewards, Strategy};

#[derive(Clone)]
pub struct HeistPreset {
    pub id: u32,
    pub name: String,
    pub stealth_only: bool,
}

impl HeistPreset {
    pub fn to_heist(
        self: &Self,
        strategy_preset: StrategyPreset,
        repeat_bonus: RepeatBonus,
    ) -> Heist {
        Heist::new(strategy_preset.strategy, self.stealth_only, repeat_bonus)
    }
}

#[derive(Clone)]
pub struct StrategyPreset {
    pub id: u32,
    pub name: String,
    pub level_one: bool,
    pub strategy: Strategy,
}

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
                    RepeatBonus(repeat),
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

#[derive(Clone, Default)]
pub struct PresetStore {
    id_counter: u32,
    data: Vec<(HeistPreset, Vec<StrategyPreset>)>,
}

impl PresetStore {
    pub fn generate_default_presets() -> Self {
        let mut id_counter = 0;
        let mut generate_id = || {
            let current_id = id_counter;
            id_counter += 1;
            current_id
        };
        let data = vec![
            (
                HeistPreset {
                    id: generate_id(),
                    name: String::from("Nightclub"),
                    stealth_only: false,
                },
                vec![
                    StrategyPreset {
                        id: generate_id(),
                        name: String::from("Full Sweep"),
                        level_one: false,
                        strategy: Strategy {
                            base_cash: 1_079_550,
                            base_xp: 840_000,
                            in_game_time: 24.0,
                            stealth: true,
                            crew: Crew(4),
                        },
                    },
                    StrategyPreset {
                        id: generate_id(),
                        name: String::from("Minimum"),
                        level_one: false,
                        strategy: Strategy {
                            base_cash: 539_550,
                            base_xp: 840_000,
                            in_game_time: 21.0,
                            stealth: true,
                            crew: Crew(4),
                        },
                    },
                ],
            ),
            (
                HeistPreset {
                    id: generate_id(),
                    name: String::from("Shadow Raid"),
                    stealth_only: true,
                },
                vec![StrategyPreset {
                    id: generate_id(),
                    name: String::from("16+ Bags"),
                    level_one: false,
                    strategy: Strategy {
                        base_cash: 4_051_539,
                        base_xp: 784_000,
                        in_game_time: 75.0,
                        stealth: false,
                        crew: Crew(4),
                    },
                }],
            ),
            (
                HeistPreset {
                    id: generate_id(),
                    name: String::from("Depot"),
                    stealth_only: false,
                },
                vec![
                    StrategyPreset {
                        id: generate_id(),
                        name: String::from("20+ Bags"),
                        level_one: false,
                        strategy: Strategy {
                            base_cash: 3_877_135,
                            base_xp: 843_100,
                            in_game_time: 80.0,
                            stealth: true,
                            crew: Crew(4),
                        },
                    },
                    StrategyPreset {
                        id: generate_id(),
                        name: String::from("Minimum"),
                        level_one: true,
                        strategy: Strategy {
                            base_cash: 485_000,
                            base_xp: 843_100,
                            in_game_time: 30.0,
                            stealth: true,
                            crew: Crew(4),
                        },
                    },
                ],
            ),
        ];
        PresetStore { id_counter, data }
    }

    pub fn get_data(self: &Self) -> &Vec<(HeistPreset, Vec<StrategyPreset>)> {
        &self.data
    }

    pub fn get_heist_strategy(
        self: &Self,
        heist_id: u32,
        strategy_id: u32,
    ) -> Result<(&HeistPreset, &StrategyPreset), ()> {
        let (heist_preset, strategy_presets) = self
            .data
            .iter()
            .find(|(heist_preset, _)| heist_preset.id == heist_id)
            .ok_or(())?;

        let strategy_preset = strategy_presets
            .iter()
            .find(|strategy_preset| strategy_preset.id == strategy_id)
            .ok_or(())?;

        Ok((heist_preset, strategy_preset))
    }

    pub fn get_heist_strategies(
        self: &Self,
        heist_id: u32,
    ) -> Result<&(HeistPreset, Vec<StrategyPreset>), ()> {
        Ok(self
            .data
            .iter()
            .find(|(heist_preset, _)| heist_preset.id == heist_id)
            .ok_or(())?)
    }
}
