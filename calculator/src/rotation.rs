use crate::{
    Heist, Level, RealTimeDurations, RealTimeEvents, Rewards,
    bonuses::{GeneralMultipliers, RepeatBonus},
    presets::{HeistPreset, StrategyPreset},
};

#[derive(Clone, Default)]
pub struct Rotation(pub Vec<(HeistPreset, StrategyPreset)>);

#[derive(Debug, Clone)]
pub struct RotationData {
    pub heist: Heist,

    pub rewards: Rewards,
    pub real_time: RealTimeEvents,

    pub cum_rewards: Rewards,
    pub cum_in_game_time: f32,
    pub cum_real_time: f32,
}

impl Rotation {
    fn to_heists(self: &Self) -> impl Iterator<Item = Heist> {
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

                Heist::new(
                    heist_preset.clone(),
                    strategy_preset.clone(),
                    // Never out of range
                    RepeatBonus::new(repeat).unwrap(),
                )
            })
    }

    pub fn final_level(self: &Self, multipliers: &GeneralMultipliers) -> Level {
        Level::from_xp(
            self.to_heists()
                .map(|heist| heist.get_reward(multipliers).xp)
                .sum(),
        )
    }

    pub fn calculate(
        self: &Self,
        multipliers: &GeneralMultipliers,
        durations: &RealTimeDurations,
    ) -> impl Iterator<Item = RotationData> {
        self.to_heists()
            .map(|heist| {
                let rewards = heist.get_reward(multipliers);

                let real_time = RealTimeEvents {
                    heist: heist.get_time() + durations.load_heist,
                    lobby: durations.load_lobby + durations.host_lobby,
                    skill_break: 0.0,
                    infamy: 0.0,
                };

                RotationData {
                    heist,

                    rewards,
                    real_time,

                    cum_rewards: Rewards { cash: 0, xp: 0 },
                    cum_in_game_time: 0.0,
                    cum_real_time: 0.0,
                }
            })
            .scan(
                (Rewards { cash: 0, xp: 0 }, 0.0, 0.0, None, None),
                |(rewards, igt, rta, last_heist, last_skill), mut data| {
                    // skill points before this heist
                    let points = Level::from_xp(rewards.xp).get_skill_points()
                        + multipliers.skill_perks.get_bonus();

                    rewards.xp += data.rewards.xp;
                    rewards.cash += data.rewards.cash;
                    data.cum_rewards = rewards.clone();

                    *igt += data.heist.get_time();
                    data.cum_in_game_time = *igt;

                    let this_id = data.heist.get_heist_preset().id;
                    if let Some(last_id) = last_heist {
                        if this_id == *last_id {
                            data.real_time.lobby = 0.0;
                        }
                    } else {
                        data.real_time.infamy = durations.infamy;
                    }
                    *last_heist = Some(this_id);

                    let skilled = data.heist.get_strategy_preset().min_skills;
                    if let Some((last_points, last_skilled)) = last_skill {
                        if *last_skilled < skilled {
                            data.real_time.skill_break = (skilled - *last_skilled) as f32
                                / durations.skill_break_skills_per_sec as f32;
                            if *last_points < skilled {
                                data.real_time.lobby = durations.load_lobby + durations.host_lobby;
                                data.real_time.skill_break += durations.skill_break_overhead;
                                // whole skill break
                                *last_skill = Some((points, skilled));
                            } else {
                                // extra skills in last break
                                *last_skilled = skilled;
                            }
                        }
                    } else {
                        data.real_time.lobby = durations.load_lobby + durations.host_lobby;
                        data.real_time.skill_break = skilled as f32
                            / durations.skill_break_skills_per_sec as f32
                            + durations.skill_break_overhead;
                        // pre-skills
                        *last_skill = Some((points, skilled));
                    };

                    *rta += data.real_time.time();
                    data.cum_real_time = *rta;

                    Some(data)
                },
            )
    }
}
