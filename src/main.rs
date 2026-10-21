use calculator::{
    GeneralMultipliers, InfamyPerk, InfamyPerks,
    data::{HeistPreset, PresetStore, Rotation, StrategyPreset},
};
use leptos::prelude::*;
use std::rc::Rc;

#[component]
fn SelectHeist(
    heist_preset: HeistPreset,
    strategy_preset: StrategyPreset,
    store: ReadSignal<PresetStore>,
    update: impl Fn(HeistPreset, StrategyPreset) + 'static,
) -> impl IntoView {
    let update = Rc::new(update);
    view! {
        <>
            <td><select
                prop:value=move || heist_preset.id
                on:change:target={
                    let update = update.clone();
                    let store = store();
                    move |ev| {
                        let id = ev.target()
                            .value()
                            .parse::<u32>()
                            .unwrap();
                        let (heist_preset, heist_strategies) = store.get_heist_strategies(id).unwrap();
                        update(heist_preset.clone(), heist_strategies[0].clone());
                    }
                }
            >
                {store().get_data().iter().map(|(heist_preset, _)| view! {
                    <option value=heist_preset.id>{heist_preset.name.clone()}</option>
                }).collect::<Vec<_>>()}
            </select></td>
            <td><select
                prop:value=move || strategy_preset.id
                on:change:target={
                    let store = store.get();
                    move |ev| {
                        let id = ev.target()
                            .value()
                            .parse::<u32>()
                            .unwrap();
                        let (_, heist_strategy) = store.get_heist_strategy(heist_preset.id, id).unwrap();
                        update(heist_preset.clone(), heist_strategy.clone());
                    }
                }
            >
                {store().get_heist_strategies(heist_preset.id).unwrap().1.iter().map(|strategy_preset| view! {
                    <option value=strategy_preset.id>{strategy_preset.name.clone()}</option>
                }).collect::<Vec<_>>()}
            </select></td>
        </>
    }
}

#[component]
fn Calculator() -> impl IntoView {
    let (rotation, set_rotation) = signal(Rotation::default());
    let (store, set_store) = signal(PresetStore::generate_default_presets());

    let default_heist = (
        store().get_data()[0].0.clone(),
        store().get_data()[0].1[0].clone(),
    );
    view! {
        <table><tbody>
            <tr>
                <td>Heist</td>
                <td>Strategy</td>
                <td>Time</td>
                <td>Stealth</td>
                <td>Repeat</td>
                <td>Base Cash</td>
                <td>Cash</td>
                <td>Cash Efficiency / m</td>
                // <td>Cum. Cash</td>
                <td>Base XP</td>
                <td>XP</td>
                <td>XP Efficiency / m</td>
                // <td>Cum. XP</td>
            </tr>
            {
                move || rotation().to_repeats().enumerate().map(|(i, (heist_preset, strategy_preset, repeat))| {
                let heist = heist_preset.to_heist(strategy_preset.clone(), repeat);
                let strat = heist.get_strategy();
                let rewards = heist.get_reward(&GeneralMultipliers {
                    group_boost: true,

                    cash_boost: true,
                    cash_gamepass: false,
                    cash_infamy_perks: InfamyPerks::Cash(InfamyPerk::new(8).unwrap()),

                    xp_boost: false,
                    xp_gamepass: false,
                    xp_infamy_perks: InfamyPerks::Xp(InfamyPerk::new(8).unwrap()),
                    xp_infamy_boost: false,
                });
                view! {
                    <tr>
                        <SelectHeist heist_preset strategy_preset store update=move |heist_preset, strategy_preset| set_rotation.update(|rot| rot.0[i] = (heist_preset, strategy_preset))/>
                        <td>{strat.in_game_time}</td>
                        <td>{strat.stealth}</td>
                        <td>{heist.get_multipliers().repeat_bonus.get_bonus()}</td>
                        <td>{strat.base_cash}</td>
                        <td>{rewards.cash}</td>
                        <td>{rewards.cash as f32 / strat.in_game_time}</td>
                        <td>{strat.base_xp}</td>
                        <td>{rewards.xp}</td>
                        <td>{rewards.xp as f32 / strat.in_game_time}</td>
                    </tr>
                }
            }).collect::<Vec<_>>()}
            <tr><button
                on:click=move |_| {
                    set_rotation.update(|rotation| rotation.0.push(default_heist.clone()));
                }
            >"+"
            </button></tr>
        </tbody></table>
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    view! {
        <div>
            <h1>"Welcome to Leptos"</h1>
            <p>"Your static site is ready!"</p>
            <button
                on:click = move |_| {
                    set_count.update(|count| *count += 1);
                }
            >
                "Click me: "
                {count}
            </button>
            <Calculator/>
        </div>
    }
}

pub fn main() {
    mount_to_body(|| view! { <App /> })
}
