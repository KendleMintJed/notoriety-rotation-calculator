use calculator::{
    RotationData,
    presets::{HeistPreset, StrategyPreset},
};

struct HeistEvent {
    rta: f32,
    cum_rta: f32,
    data: RotationData,
}

fn find_best_rotation(
    choises: Vec<(HeistPreset, StrategyPreset)>,
    predicate: impl Fn(HeistEvent, HeistEvent) -> bool,
) {
}

fn maximize_xp(old: HeistEvent, new: HeistEvent) -> bool {
    new.cum_rta > old.cum_rta
}

fn main() {
    println!("Hello, world!");
}
