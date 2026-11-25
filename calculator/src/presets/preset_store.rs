use crate::{
    Strategy,
    bonuses::Crew,
    presets::{HeistPreset, StrategyPreset},
};

#[derive(Clone, Default)]
pub struct PresetStore {
    _id_counter: u32,
    data: Vec<(HeistPreset, Vec<StrategyPreset>)>,
}

impl PresetStore {
    pub fn generate_default_presets() -> Self {
        let mut _id_counter = 0;
        let mut generate_id = || {
            let current_id = _id_counter;
            _id_counter += 1;
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
                        min_skills: 100,
                        strategy: Strategy {
                            base_cash: 1_079_550,
                            base_xp: 840_000,
                            in_game_time: 24.0,
                            stealth: true,
                            crew: Crew::new(4).unwrap(),
                        },
                    },
                    StrategyPreset {
                        id: generate_id(),
                        name: String::from("Full Sweep"),
                        min_skills: 33,
                        strategy: Strategy {
                            base_cash: 1_079_550,
                            base_xp: 840_000,
                            in_game_time: 30.0,
                            stealth: true,
                            crew: Crew::new(4).unwrap(),
                        },
                    },
                    StrategyPreset {
                        id: generate_id(),
                        name: String::from("Minimum"),
                        min_skills: 100,
                        strategy: Strategy {
                            base_cash: 539_550,
                            base_xp: 840_000,
                            in_game_time: 21.0,
                            stealth: true,
                            crew: Crew::new(4).unwrap(),
                        },
                    },
                    StrategyPreset {
                        id: generate_id(),
                        name: String::from("Minimum"),
                        min_skills: 33,
                        strategy: Strategy {
                            base_cash: 539_550,
                            base_xp: 840_000,
                            in_game_time: 25.0,
                            stealth: true,
                            crew: Crew::new(4).unwrap(),
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
                vec![
                    StrategyPreset {
                        id: generate_id(),
                        name: String::from("16+ Bags"),
                        min_skills: 70,
                        strategy: Strategy {
                            base_cash: 4_051_539,
                            base_xp: 784_000,
                            in_game_time: 75.0,
                            stealth: false,
                            crew: Crew::new(4).unwrap(),
                        },
                    },
                    StrategyPreset {
                        id: generate_id(),
                        name: String::from("16+ Bags"),
                        min_skills: 32,
                        strategy: Strategy {
                            base_cash: 4_051_539,
                            base_xp: 784_000,
                            in_game_time: 85.0,
                            stealth: false,
                            crew: Crew::new(4).unwrap(),
                        },
                    },
                ],
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
                        min_skills: 100,
                        strategy: Strategy {
                            base_cash: 3_877_135,
                            base_xp: 843_100,
                            in_game_time: 80.0,
                            stealth: true,
                            crew: Crew::new(4).unwrap(),
                        },
                    },
                    StrategyPreset {
                        id: generate_id(),
                        name: String::from("Minimum"),
                        min_skills: 33,
                        strategy: Strategy {
                            base_cash: 485_000,
                            base_xp: 843_100,
                            in_game_time: 30.0,
                            stealth: true,
                            crew: Crew::new(4).unwrap(),
                        },
                    },
                ],
            ),
            (
                HeistPreset {
                    id: generate_id(),
                    name: String::from("Rush Hour"),
                    stealth_only: false,
                },
                vec![StrategyPreset {
                    id: generate_id(),
                    name: String::from("Minimum"),
                    min_skills: 100,
                    strategy: Strategy {
                        base_cash: 475_000,
                        base_xp: 1_288_000,
                        in_game_time: 53.0,
                        stealth: true,
                        crew: Crew::new(4).unwrap(),
                    },
                }],
            ),
            (
                HeistPreset {
                    id: generate_id(),
                    name: String::from("Diamond Store"),
                    stealth_only: false,
                },
                vec![StrategyPreset {
                    id: generate_id(),
                    name: String::from("Minimum"),
                    min_skills: 30,
                    strategy: Strategy {
                        base_cash: 365_200,
                        base_xp: 352_000,
                        in_game_time: 11.0,
                        stealth: false,
                        crew: Crew::new(4).unwrap(),
                    },
                }],
            ),
        ];
        PresetStore { _id_counter, data }
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

    pub fn get_default(self: &Self) -> (&HeistPreset, &StrategyPreset) {
        (&self.data[0].0, &self.data[0].1[0])
    }
}
