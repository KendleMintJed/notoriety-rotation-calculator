use leptos::prelude::*;

use calculator::data::{HeistPreset, PresetStore, StrategyPreset};
use std::rc::Rc;

#[component]
pub fn SelectHeist(
    heist_preset: HeistPreset,
    strategy_preset: StrategyPreset,
    store: ReadSignal<PresetStore>,
    update: impl Fn(HeistPreset, StrategyPreset) + 'static,
) -> impl IntoView {
    let update = Rc::new(update);
    view! {
        <td><select
            prop:value=move || heist_preset.id
            on:change:target={
                let update = update.clone();
                move |ev| {
                    let store = store();
                    let id = ev.target()
                        .value()
                        .parse::<u32>()
                        .unwrap();
                    let (heist_preset, heist_strategies) = store.get_heist_strategies(id).unwrap();
                    update(heist_preset.clone(), heist_strategies[0].clone());
                }
            }
        >
            {move || store().get_data().iter().map(|(heist_preset, _)| view! {
                <option value=heist_preset.id>{heist_preset.name.clone()}</option>
            }).collect::<Vec<_>>()}
        </select></td>
        <td><select
            prop:value=move || strategy_preset.id
            on:change:target=move |ev| {
                let store = store.get();
                let id = ev.target()
                    .value()
                    .parse::<u32>()
                    .unwrap();
                let (_, heist_strategy) = store.get_heist_strategy(heist_preset.id, id).unwrap();
                update(heist_preset.clone(), heist_strategy.clone());
            }

        >
            {move || store().get_heist_strategies(heist_preset.id).unwrap().1.iter().map(|strategy_preset| view! {
                <option value=strategy_preset.id>{strategy_preset.name.clone()}</option>
            }).collect::<Vec<_>>()}
        </select></td>
    }
}
