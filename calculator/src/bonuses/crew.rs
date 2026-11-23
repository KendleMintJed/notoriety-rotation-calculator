#[derive(Clone)]
pub struct Crew(u8);

impl Crew {
    pub fn new(players: u8) -> Result<Self, ()> {
        if players < 1 || players > 4 {
            Err(())
        } else {
            Ok(Self(players))
        }
    }

    pub fn get_bonus(self: &Self) -> f32 {
        (self.0 - 1) as f32 * 0.05
    }
}
