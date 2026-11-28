use calculator::{
    RealTimeDurations, Rotation, RotationData,
    bonuses::GeneralMultipliers,
    presets::{HeistPreset, PresetStore, StrategyPreset},
};

struct HeistEvent {
    rta: f32,
    cum_rta: f32,
    data: RotationData,
}

fn rotation_combinations(
    mut rotation: Rotation,
    choises: &[(HeistPreset, StrategyPreset)],
    multipliers: &GeneralMultipliers,
) -> Vec<Rotation> {
    let mut rotations = vec![];
    loop {
        let level = rotation.final_level(multipliers);

        if level.get_level() == 100 {
            break;
        }

        if choises.len() > 1
            && level.get_skill_points() + multipliers.skill_perks.get_bonus()
                >= choises[1].1.min_skills
        {
            rotations.append(&mut rotation_combinations(
                rotation.clone(),
                &choises[1..],
                multipliers,
            ));
        }
        rotation.0.push(choises[0].clone());
    }

    rotations.push(rotation);
    rotations
}

fn find_best_rotation(
    mut choises: Vec<(HeistPreset, StrategyPreset)>,
    multipliers: &GeneralMultipliers,
    durations: &RealTimeDurations,
) {
    choises.sort_by_key(|x| x.1.min_skills);
    let rotations = rotation_combinations(Rotation(Vec::new()), choises.as_slice(), &multipliers);
    let mut values = rotations
        .iter()
        .map(|rotation| {
            let events = rotation.calculate(multipliers, durations);
            let (cum_cash, cum_rta) = if let Some(last) = events.last() {
                (last.cum_rewards.cash, last.cum_real_time)
            } else {
                (0, 0.0)
            };
            (rotation, cum_cash, cum_rta)
        })
        .collect::<Vec<_>>();
    values.sort_by(|a, b| {
        let a = a.1 as f32 / a.2;
        let b = b.1 as f32 / b.2;
        if a == b {
            std::cmp::Ordering::Equal
        } else if a < b {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });
    let mut time = f32::MAX;
    let values = values
        .iter()
        .filter(|(_, _, cum_rta)| {
            if *cum_rta < time {
                time = *cum_rta;
                true
            } else {
                false
            }
        })
        .map(|(rot, cum_cash, cum_rta)| {
            format!("{}: {}", *cum_cash as f32 / cum_rta, display_rotation(rot))
        })
        .collect::<Vec<_>>();

    for value in values {
        println!("{}", value);
    }
}

fn display_rotation(rotation: &Rotation) -> String {
    format!(
        "{:?}",
        rotation
            .0
            .iter()
            .fold(
                Vec::new(),
                |mut acc: Vec<(u32, u32, String, i32)>, (heist_preset, strategy_preset)| {
                    if let Some((i, _)) = acc
                        .iter()
                        .enumerate()
                        .filter(|(i, v)| v.0 == heist_preset.id && v.1 == strategy_preset.id)
                        .next()
                    {
                        acc[i].3 += 1;
                    } else {
                        acc.push((
                            heist_preset.id,
                            strategy_preset.id,
                            format!(
                                "{} {} {}p ({}pts)",
                                heist_preset.name,
                                strategy_preset.name,
                                strategy_preset.strategy.crew.get_players(),
                                strategy_preset.min_skills
                            ),
                            1,
                        ));
                    }
                    acc
                },
            )
            .into_iter()
            .map(|x| format!("{} x{}", x.2, x.3))
            .collect::<Vec<_>>()
    )
}

fn display_rotation_combinations(
    mut choises: Vec<(HeistPreset, StrategyPreset)>,
    multipliers: &GeneralMultipliers,
) {
    choises.sort_by_key(|x| x.1.min_skills);
    let rotations = rotation_combinations(Rotation(Vec::new()), choises.as_slice(), &multipliers);
    rotations
        .iter()
        .map(|rotation| display_rotation(rotation))
        .for_each(|rotation| println!("{}", rotation));
}

fn maximize_xp(
    (li, left): (usize, HeistEvent),
    (ri, right): (usize, HeistEvent),
) -> (usize, HeistEvent) {
    if right.cum_rta > left.cum_rta {
        (li, left)
    } else {
        (ri, right)
    }
}

fn main() {
    let store = PresetStore::generate_default_presets();
    let data = store.get_data();

    let heists = vec![
        // Nigthclub
        (data[0].0.clone(), data[0].1[0].clone()), // fs full skills
        (data[0].0.clone(), data[0].1[1].clone()), // fs lvl1 skills
        // Shadow Raid
        (data[1].0.clone(), data[1].1[0].clone()), // fs full skills
        (data[1].0.clone(), data[1].1[1].clone()), // fs lvl1 skills
        // Depot
        (data[2].0.clone(), data[2].1[0].clone()), // fs full skills
        (data[2].0.clone(), data[2].1[1].clone()), // min lvl1 skills
        // Rush Hour
        (data[3].0.clone(), data[3].1[0].clone()),
        // Diamond Store
        (data[4].0.clone(), data[4].1[0].clone()),
        // Art Gallery
        (data[5].0.clone(), data[5].1[0].clone()),
    ];

    let mut multipliers = GeneralMultipliers::default();
    multipliers.xp_gamepass = true;
    multipliers.cash_gamepass = true;

    find_best_rotation(heists, &multipliers, &RealTimeDurations::default());
}
