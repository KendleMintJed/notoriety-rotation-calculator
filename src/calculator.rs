use calculator::{GeneralMultipliers, InfamyPerk, InfamyPerks};
use leptos::prelude::*;

use ::calculator::data::{PresetStore, Rotation};

mod select_multipliers;
use select_multipliers::SelectMultipliers;

mod rotation_table;
use rotation_table::RotationTable;

#[component]
pub fn Calculator() -> impl IntoView {
    let (rotation, set_rotation) = signal(Rotation::default());
    let multipliers = RwSignal::new(GeneralMultipliers::default());
    let (store, _) = signal(PresetStore::generate_default_presets());
    let infamy_skills = RwSignal::new(InfamyPerks::SkillPoints(InfamyPerk::new(8).unwrap()));

    view! {
        <SelectMultipliers multipliers infamy_skills />
        <RotationTable
            rotation set_rotation store
            multipliers=multipliers.read_only()
            infamy_skills=infamy_skills.read_only()
        />
    }
}
