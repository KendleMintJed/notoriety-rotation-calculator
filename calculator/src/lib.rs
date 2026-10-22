pub mod data;

pub struct InfamyPerk(u8);

impl InfamyPerk {
    pub fn new(level: u8) -> Result<Self, ()> {
        if level > 8 { Err(()) } else { Ok(Self(level)) }
    }
}

pub enum InfamyPerks {
    Cash(InfamyPerk),
    Xp(InfamyPerk),
    SkillPoints(InfamyPerk),
}

impl InfamyPerks {
    pub fn get_bonus(self: &Self) -> f32 {
        match self {
            InfamyPerks::Cash(level) => (level.0 as f32) / 10.0,

            InfamyPerks::Xp(level) => {
                let five_buffs: f32 = std::cmp::min(level.0, 4).into();
                let ten_buffs: f32 = level.0.checked_sub(4).unwrap_or(0).into();
                five_buffs * 0.05 + ten_buffs * 0.1
            }

            InfamyPerks::SkillPoints(level) => {
                let three_buffs = std::cmp::min(level.0, 4);
                let five_buffs = level.0.checked_sub(4).unwrap_or(0);
                (three_buffs * 3 + five_buffs * 5).into()
            }
        }
    }
}

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

pub struct GeneralMultipliers {
    pub group_boost: bool,

    pub cash_boost: bool,
    pub cash_gamepass: bool,
    pub cash_infamy_perks: InfamyPerks,

    pub xp_boost: bool,
    pub xp_gamepass: bool,
    pub xp_infamy_perks: InfamyPerks,
    pub xp_infamy_boost: bool,
}

pub struct SpesificMultipliers {
    pub repeat_bonus: RepeatBonus,
    pub stealth_bonus: bool,
}

pub struct Multipliers {
    pub general: GeneralMultipliers,
    pub spesific: SpesificMultipliers,
}

#[derive(Clone)]
pub struct Strategy {
    pub base_cash: u32,
    pub base_xp: u32,
    pub in_game_time: f32,
    pub stealth: bool,
    pub crew: Crew,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rewards {
    pub cash: u32,
    pub xp: u32,
}

pub struct Heist {
    strategy: Strategy,
    multipliers: SpesificMultipliers,
}

impl Heist {
    pub fn new(strategy: Strategy, stealth_only: bool, repeat_bonus: RepeatBonus) -> Heist {
        let stealth = strategy.stealth;
        Heist {
            strategy,
            multipliers: SpesificMultipliers {
                repeat_bonus,
                stealth_bonus: stealth && !stealth_only,
            },
        }
    }
    pub fn get_reward(self: &Self, general: &GeneralMultipliers) -> Rewards {
        fn mul(lhs: bool, rhs: f32) -> f32 {
            if lhs { rhs } else { 0.0 }
        }

        let spesific = &self.multipliers;
        let strategy = &self.strategy;

        let cash_multiplier = 1.0
            + mul(general.group_boost, 0.15)
            + mul(general.cash_boost, 1.0)
            + mul(general.cash_gamepass, 1.0)
            + general.cash_infamy_perks.get_bonus()
            + spesific.repeat_bonus.get_bonus();

        let xp_multiplier = 1.0
            + mul(general.group_boost, 0.15)
            + strategy.crew.get_bonus()
            + mul(general.xp_boost, 1.0)
            + mul(general.xp_gamepass, 1.0)
            + general.xp_infamy_perks.get_bonus()
            + mul(general.xp_infamy_boost, 0.15)
            + spesific.repeat_bonus.get_bonus()
            + mul(spesific.stealth_bonus, 0.1);

        let cash = (strategy.base_cash as f32 * cash_multiplier) as u32;
        let xp = (strategy.base_xp as f32 * xp_multiplier) as u32;

        Rewards { cash, xp }
    }
    pub fn get_strategy(self: &Self) -> &Strategy {
        &self.strategy
    }
    pub fn get_multipliers(self: &Self) -> &SpesificMultipliers {
        &self.multipliers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nc_fs_generation() {
        let strategy = Strategy {
            base_xp: 840_000,
            base_cash: 1_079_550,
            in_game_time: 24.0,
            stealth: true,
            crew: Crew(3),
        };
        let general_multipliers = GeneralMultipliers {
            group_boost: true,

            cash_boost: true,
            cash_gamepass: true,
            cash_infamy_perks: InfamyPerks::Cash(InfamyPerk(8)),

            xp_boost: true,
            xp_gamepass: true,
            xp_infamy_perks: InfamyPerks::Xp(InfamyPerk(8)),
            xp_infamy_boost: false,
        };
        let spesific_multipliers = SpesificMultipliers {
            repeat_bonus: RepeatBonus(0),
            stealth_bonus: true,
        };
        let heist = Heist {
            strategy,
            multipliers: spesific_multipliers,
        };

        assert_eq!(
            heist.get_reward(&general_multipliers),
            Rewards {
                cash: 5_343_772,
                xp: 4_200_000,
            }
        )
    }
}
