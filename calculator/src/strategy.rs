use crate::bonuses::Crew;

#[derive(Debug, Clone)]
pub struct Strategy {
    pub base_cash: u32,
    pub base_xp: u32,
    pub in_game_time: f32,
    pub stealth: bool,
    pub crew: Crew,
}
