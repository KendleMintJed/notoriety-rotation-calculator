use crate::RotationData;

#[derive(Debug, Clone)]
pub struct RealTimeDurations {
    pub load_heist: f32,
    pub load_lobby: f32,
    pub host_lobby: f32,
    pub infamy: f32,
    pub skill_break_overhead: f32,
    pub skill_break_skills_per_sec: u8,
}

impl Default for RealTimeDurations {
    fn default() -> Self {
        RealTimeDurations {
            load_heist: 20.0,
            load_lobby: 17.0,
            host_lobby: 15.0,
            infamy: 10.0,
            skill_break_overhead: 4.0,
            skill_break_skills_per_sec: 18,
        }
    }
}

pub enum RealTimeEvents {
    Heist(RotationData),
    Lobby,
    SkillBreak(u8),
    Infamy,
}

impl RealTimeEvents {
    pub fn get_time(self: &Self, durations: &RealTimeDurations) -> f32 {
        use RealTimeEvents::*;
        match self {
            Heist(heist) => heist.strategy_preset.strategy.in_game_time + durations.load_heist,
            Lobby => durations.load_lobby + durations.host_lobby,
            SkillBreak(skills) => {
                durations.skill_break_overhead
                    + (*skills as f32 / durations.skill_break_skills_per_sec as f32)
            }
            Infamy => durations.infamy,
        }
    }
}
