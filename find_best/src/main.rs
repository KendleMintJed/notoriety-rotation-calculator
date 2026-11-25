use calculator::{
    Level, RealTimeDurations, RealTimeEvents, Rotation, RotationData,
    bonuses::{GeneralMultipliers, perks::SkillPerk},
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
    skill_perks: &SkillPerk,
) -> Vec<Rotation> {
    let mut rotations = vec![];
    loop {
        let last = rotation.calculate(multipliers).last();
        let level = if let Some(last) = last {
            last.cum_level
        } else {
            Level::from_xp(0)
        };

        if level.get_level() == 100 {
            break;
        }

        if choises.len() > 1
            && level.get_skill_points() + skill_perks.get_bonus() >= choises[1].1.min_skills
        {
            rotations.append(&mut rotation_combinations(
                rotation.clone(),
                &choises[1..],
                multipliers,
                skill_perks,
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
    skill_perks: &SkillPerk,
    durations: &RealTimeDurations,
    predicate: impl Fn((usize, HeistEvent), (usize, HeistEvent)) -> (usize, HeistEvent) + Clone,
) {
    choises.sort_by_key(|x| x.1.min_skills);
    let rotations = rotation_combinations(
        Rotation(Vec::new()),
        choises.as_slice(),
        &multipliers,
        skill_perks,
    );
    let best_rotation = rotations
        .iter()
        .enumerate()
        .map(|(i, rotation)| {
            (
                i,
                rotation
                    .to_real_time(multipliers, skill_perks)
                    .fold(0.0, |cum_rta, event| cum_rta + event.get_time(durations)),
            )
        })
        .reduce(|(i1, v1), (i2, v2)| if v1 > v2 { (i2, v2) } else { (i1, v1) });
    println!("{}", display_rotation(&rotations[best_rotation.unwrap().0]))
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
                                "{} {}p ({}pts)",
                                heist_preset.name,
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
    skill_perks: &SkillPerk,
) {
    choises.sort_by_key(|x| x.1.min_skills);
    let rotations = rotation_combinations(
        Rotation(Vec::new()),
        choises.as_slice(),
        &multipliers,
        skill_perks,
    );
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
        (data[0].0.clone(), data[0].1[2].clone()), // min full skills
        (data[0].0.clone(), data[0].1[3].clone()), // min lvl1 skills
        // Depot
        (data[2].0.clone(), data[2].1[1].clone()), // min lvl1 skills
        // Rush Hour
        (data[3].0.clone(), data[3].1[0].clone()),
        // Diamond Store
        (data[4].0.clone(), data[4].1[0].clone()),
    ];

    find_best_rotation(
        heists,
        &GeneralMultipliers::default(),
        &SkillPerk::default(),
        &RealTimeDurations::default(),
        maximize_xp,
    );
}
