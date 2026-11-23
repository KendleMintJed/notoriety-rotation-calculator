pub struct RealTimeEvents {
    pub load_heist: f32,
    pub load_lobby: f32,
    pub host_lobby: f32,
    pub skill_break_overhead: f32,
    pub skill_break_skills_per_sec: u8,
}

impl Default for RealTimeEvents {
    fn default() -> Self {
        RealTimeEvents {
            load_heist: 20.0,
            load_lobby: 17.0,
            host_lobby: 15.0,
            skill_break_overhead: 4.0,
            skill_break_skills_per_sec: 18,
        }
    }
}
