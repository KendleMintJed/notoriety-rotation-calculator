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

    pub fn xp_next(level: u8) -> u32 {
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
        assert_eq!(level.xp, 27_181_305)
    }
}
