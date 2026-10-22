pub mod data;

#[derive(Clone)]
pub struct InfamyPerk(u8);

impl InfamyPerk {
    pub fn new(level: u8) -> Result<Self, ()> {
        if level > 8 { Err(()) } else { Ok(Self(level)) }
    }
}

impl Default for InfamyPerk {
    fn default() -> Self {
        InfamyPerk(8)
    }
}

#[derive(Clone)]
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

    pub fn get_level(self: &Self) -> u8 {
        match self {
            Self::Cash(perk) => perk.0,
            Self::Xp(perk) => perk.0,
            Self::SkillPoints(perk) => perk.0,
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

#[derive(Clone)]
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

fn mul(lhs: bool, rhs: f32) -> f32 {
    if lhs { rhs } else { 0.0 }
}

impl GeneralMultipliers {
    pub fn get_cash_boost(self: &Self) -> f32 {
        mul(self.group_boost, 0.15)
            + mul(self.cash_boost, 1.0)
            + mul(self.cash_gamepass, 1.0)
            + self.cash_infamy_perks.get_bonus()
    }

    pub fn get_xp_boost(self: &Self) -> f32 {
        mul(self.group_boost, 0.15)
            + mul(self.xp_boost, 1.0)
            + mul(self.xp_gamepass, 1.0)
            + self.xp_infamy_perks.get_bonus()
            + mul(self.xp_infamy_boost, 0.15)
    }
}

impl Default for GeneralMultipliers {
    fn default() -> Self {
        GeneralMultipliers {
            group_boost: true,
            cash_boost: false,
            cash_gamepass: false,
            cash_infamy_perks: InfamyPerks::Cash(InfamyPerk::default()),
            xp_boost: false,
            xp_gamepass: false,
            xp_infamy_perks: InfamyPerks::Xp(InfamyPerk::default()),
            xp_infamy_boost: false,
        }
    }
}

pub struct SpesificMultipliers {
    pub repeat_bonus: RepeatBonus,
    pub stealth_bonus: bool,
}

impl SpesificMultipliers {
    pub fn get_cash_boost(self: &Self) -> f32 {
        self.repeat_bonus.get_bonus()
    }
    pub fn get_xp_boost(self: &Self) -> f32 {
        self.repeat_bonus.get_bonus() + mul(self.stealth_bonus, 0.1)
    }
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

pub struct Level {
    level: u8,
    xp: u32,
}

impl Level {
    pub fn get_level(self: &Self) -> u8 {
        self.level
    }

    pub fn get_xp(self: &Self) -> u32 {
        self.xp
    }

    pub fn get_skill_points(self: &Self) -> u8 {
        self.level + self.level / 10 * 3
    }

    fn xp_next(level: u8) -> u32 {
        let level = level as f64;
        (level * 1018.93 + level.powf(2.976664)).round() as u32
    }

    pub fn max_xp() -> u32 {
        (1..100).map(Self::xp_next).sum()
    }

    pub fn from_level(level: u8) -> Level {
        if level < 2 {
            return Level { level: 1, xp: 0 };
        }
        Level {
            level,
            xp: (1..level).map(Self::xp_next).sum(),
        }
    }

    pub fn from_xp(xp: u32) -> Level {
        if xp >= Self::max_xp() {
            return Level {
                level: 100,
                xp: Self::max_xp(),
            };
        }

        let mut level = 1;
        let mut excess_xp = xp;
        loop {
            let xp_next = Level::xp_next(level);
            if xp_next > excess_xp {
                break;
            }
            level += 1;
            excess_xp -= xp_next;
        }
        Level { level, xp }
    }

    pub fn add_xp(self: &Self, xp: u32) -> Level {
        Self::from_xp(self.xp + xp)
    }
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
        let spesific = &self.multipliers;
        let strategy = &self.strategy;

        let cash_multiplier = 1.0 + general.get_cash_boost() + spesific.get_cash_boost();

        let xp_multiplier =
            1.0 + general.get_xp_boost() + spesific.get_xp_boost() + strategy.crew.get_bonus();

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
    fn test_xp_next() {
        let levels = [
            1_020, 2_046, 3_083, 4_138, 5_215, 6_321, 7_460, 8_639, 9_863, 11_137, 12_467, 13_858,
            15_315, 16_845, 18_452, 20_142, 21_920, 23_792, 25_763, 27_838, 30_023, 32_323, 34_744,
            37_290, 39_968, 42_781, 45_737, 48_840, 52_095, 55_508, 59_084, 62_828, 66_746, 70_843,
            75_124, 79_595, 84_260, 89_126, 94_197, 99_478, 104_976, 110_695, 116_640, 122_817,
            129_231, 135_887, 142_791, 149_948, 157_363, 165_041, 172_987, 181_207, 189_706,
            198_490, 207_563, 216_931, 226_598, 236_571, 246_854, 257_453, 268_373, 279_619,
            291_196, 303_110, 315_365, 327_967, 340_922, 354_234, 367_908, 381_950, 396_366,
            411_159, 426_336, 441_901, 457_861, 474_219, 490_981, 508_153, 525_740, 543_746,
            562_177, 581_038, 600_335, 620_072, 640_255, 660_888, 681_978, 703_529, 725_546,
            748_035, 771_001, 794_449, 818_383, 842_810, 867_735, 893_162, 919_097, 945_545,
            972_511,
        ];
        assert_eq!(levels.iter().sum::<u32>(), 27_181_305);
        (1..100)
            .map(Level::xp_next)
            .enumerate()
            .for_each(|(i, v)| assert_eq!(v, levels[i]));
    }

    #[test]
    fn test_from_level() {
        let level = Level::from_level(100);
        assert_eq!(level.0, 27_181_305)
    }

    #[test]
    fn test_nc_fs_generation() {
        let strategy = Strategy {
            base_xp: 840_000,
            base_cash: 1_079_550,
            in_game_time: 24.0,
            stealth: true,
            crew: Crew(4),
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
