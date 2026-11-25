use crate::{
    Heist, Level, RealTimeDurations, RealTimeEvents, Rewards,
    bonuses::{GeneralMultipliers, RepeatBonus, perks::SkillPerk},
    presets::{HeistPreset, StrategyPreset},
};

#[derive(Clone, Default)]
pub struct Rotation(pub Vec<(HeistPreset, StrategyPreset)>);

#[derive(Debug, Clone)]
pub struct RotationData {
    pub heist_preset: HeistPreset,
    pub strategy_preset: StrategyPreset,
    pub heist: Heist,
    pub rewards: Rewards,
    pub cum_rewards: Rewards,
    pub cum_time: f32,
    pub cum_level: Level,
}

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

    pub fn calculate(
        self: &Self,
        multipliers: &GeneralMultipliers,
    ) -> impl Iterator<Item = RotationData> {
        self.to_repeats()
            .map(|(heist_preset, strategy_preset, repeat_bonus)| {
                let heist = heist_preset.to_heist(strategy_preset.clone(), repeat_bonus);
                let rewards = heist.get_reward(multipliers);
                RotationData {
                    heist_preset,
                    strategy_preset,
                    heist,
                    rewards: rewards.clone(),
                    cum_rewards: rewards,
                    cum_time: 0.0,
                    cum_level: Level::from_xp(0),
                }
            })
            .scan(
                (Rewards { cash: 0, xp: 0 }, 0.0),
                |(reward_state, time_state), mut rotation_data| {
                    reward_state.cash += rotation_data.rewards.cash;
                    reward_state.xp += rotation_data.rewards.xp;

                    *time_state += rotation_data.strategy_preset.strategy.in_game_time;

                    rotation_data.cum_rewards = reward_state.clone();
                    rotation_data.cum_time = *time_state;
                    rotation_data.cum_level = Level::from_xp(reward_state.xp);

                    Some(rotation_data)
                },
            )
    }

    pub fn to_real_time(
        self: &Self,
        multipliers: &GeneralMultipliers,
        skill_perks: &SkillPerk,
    ) -> impl Iterator<Item = RealTimeEvents> {
        self.calculate(multipliers)
            .scan(
                (None, 0u8, Level::from_xp(0)),
                move |(heist_opt, skill_points, pre_level), data| {
                    let mut events = vec![];
                    let mut lobby = false;

                    // Change Heist?
                    if let Some(heist) = heist_opt {
                        if *heist != data.heist_preset.id {
                            events.push(RealTimeEvents::Lobby);
                            *heist_opt = Some(data.heist_preset.id);
                            lobby = true;
                        }
                    } else {
                        events.push(RealTimeEvents::Lobby);
                        events.push(RealTimeEvents::Infamy);
                        *heist_opt = Some(data.heist_preset.id);
                        lobby = true;
                    }

                    // Skill break?
                    if data.strategy_preset.min_skills > *skill_points {
                        if !lobby {
                            events.push(RealTimeEvents::Lobby);
                        }
                        events.push(RealTimeEvents::SkillBreak(
                            data.strategy_preset.min_skills - *skill_points,
                        ));
                        *skill_points = pre_level.get_skill_points() + skill_perks.get_bonus();
                    }

                    *pre_level = data.cum_level.clone();
                    events.push(RealTimeEvents::Heist(data));

                    Some(events)
                },
            )
            .flatten()
    }

    pub fn to_cum_real_time(
        self: &Self,
        multipliers: &GeneralMultipliers,
        skill_perks: &SkillPerk,
        durations: &RealTimeDurations,
    ) -> impl Iterator<Item = (f32, RealTimeEvents)> {
        self.to_real_time(multipliers, skill_perks)
            .scan(0.0, |cum_rta, event| {
                let rta = event.get_time(durations);
                *cum_rta += rta;
                Some((*cum_rta, event))
            })
    }
}
