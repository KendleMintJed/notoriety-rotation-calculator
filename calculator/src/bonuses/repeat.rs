#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepeatBonus(u8);

impl RepeatBonus {
    pub fn new(stage: u8) -> Result<Self, ()> {
        if stage > 7 { Err(()) } else { Ok(Self(stage)) }
    }

    pub fn get_bonus(self: &Self) -> f32 {
        match self.0 {
            0 => 1.0,
            1 => 0.75,
            2 => 0.5,
            3 => 0.25,
            4 => 0.15,
            5 => 0.10,
            6 => 0.05,
            7 => 0.0,
            _ => panic!("Corrupted RepeatBonus: {}", self.0),
        }
    }
}
