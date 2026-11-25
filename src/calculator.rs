use calculator::{
    RealTimeDurations, Rotation,
    bonuses::{GeneralMultipliers, perks::SkillPerk},
    presets::PresetStore,
};
use leptos::prelude::*;

mod select_multipliers;
use select_multipliers::SelectMultipliers;

mod select_durations;
use select_durations::SelectDurations;

mod rotation_table;
use rotation_table::RotationTable;

#[component]
pub fn Calculator() -> impl IntoView {
    let (rotation, set_rotation) = signal(Rotation::default());
    let multipliers = RwSignal::new(GeneralMultipliers::default());
    let durations = RwSignal::new(RealTimeDurations::default());
    let (store, _) = signal(PresetStore::generate_default_presets());
    let infamy_skills = RwSignal::new(SkillPerk::default());

    view! {
        <SelectMultipliers multipliers infamy_skills /><br/><br/>
        <SelectDurations durations/><br/><br/>
        <RotationTable
            rotation set_rotation store
            multipliers=multipliers.read_only()
            durations=durations.read_only()
            infamy_skills=infamy_skills.read_only()
        />
    }
}
