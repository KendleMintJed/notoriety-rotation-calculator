use calculator::GeneralMultipliers;
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

    view! {
        <SelectMultipliers multipliers />
        <RotationTable
            rotation set_rotation store
            multipliers=multipliers.read_only()
        />
    }
}
