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

#[derive(Debug, Clone)]
pub struct RealTimeEvents {
    pub heist: f32,
    pub lobby: f32,
    pub skill_break: f32,
    pub infamy: f32,
}

impl RealTimeEvents {
    pub fn time(&self) -> f32 {
        self.heist + self.lobby + self.skill_break + self.infamy
    }

    pub fn calculate_heist(in_game_time: f32, durations: &RealTimeDurations) -> f32 {
        in_game_time + durations.load_heist
    }
    pub fn set_heist(self: &mut Self, in_game_time: f32, durations: &RealTimeDurations) {
        self.heist = Self::calculate_heist(in_game_time, durations);
    }

    pub fn calculate_lobby(durations: &RealTimeDurations) -> f32 {
        durations.load_lobby + durations.host_lobby
    }
    pub fn set_lobby(self: &mut Self, durations: &RealTimeDurations) {
        self.lobby = Self::calculate_lobby(durations);
    }

    pub fn calculate_skill_break(skills: &u8, durations: &RealTimeDurations) -> f32 {
        durations.skill_break_overhead
            + (*skills as f32 / durations.skill_break_skills_per_sec as f32)
    }
    pub fn set_skill_break(self: &mut Self, skills: &u8, durations: &RealTimeDurations) {
        self.skill_break = Self::calculate_skill_break(skills, durations);
    }

    pub fn calculate_infamy(durations: &RealTimeDurations) -> f32 {
        durations.infamy
    }
    pub fn set_infamy(self: &mut Self, durations: &RealTimeDurations) {
        self.infamy = Self::calculate_infamy(durations);
    }
}
