#[derive(Clone)]
struct InfamyPerk(u8);

impl InfamyPerk {
    pub fn get_level(self: &Self) -> u8 {
        self.0
    }

    pub fn new(level: u8) -> Result<Self, ()> {
        if level > 8 { Err(()) } else { Ok(Self(level)) }
    }
}

impl Default for InfamyPerk {
    fn default() -> Self {
        InfamyPerk(8)
    }
}

#[derive(Clone, Default)]
pub struct CashPerk(InfamyPerk);

impl CashPerk {
    pub fn new(level: u8) -> Result<Self, ()> {
        Ok(Self(InfamyPerk::new(level)?))
    }

    pub fn get_level(self: &Self) -> u8 {
        self.0.get_level()
    }

    pub fn get_bonus(self: &Self) -> f32 {
        (self.0.0 as f32) / 10.0
    }
}

#[derive(Clone, Default)]
pub struct XpPerk(InfamyPerk);

impl XpPerk {
    pub fn new(level: u8) -> Result<Self, ()> {
        Ok(Self(InfamyPerk::new(level)?))
    }

    pub fn get_level(self: &Self) -> u8 {
        self.0.get_level()
    }

    pub fn get_bonus(self: &Self) -> f32 {
        let five_buffs: f32 = std::cmp::min(self.0.0, 4).into();
        let ten_buffs: f32 = self.0.0.checked_sub(4).unwrap_or(0).into();
        five_buffs * 0.05 + ten_buffs * 0.1
    }
}

#[derive(Clone, Default)]
pub struct SkillPerk(InfamyPerk);

impl SkillPerk {
    pub fn new(level: u8) -> Result<Self, ()> {
        Ok(Self(InfamyPerk::new(level)?))
    }

    pub fn get_level(self: &Self) -> u8 {
        self.0.get_level()
    }

    pub fn get_bonus(self: &Self) -> u8 {
        let three_buffs = std::cmp::min(self.0.0, 4);
        let five_buffs = self.0.0.checked_sub(4).unwrap_or(0);
        three_buffs * 3 + five_buffs * 5
    }
}
