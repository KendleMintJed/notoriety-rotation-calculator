use calculator::{
    GeneralMultipliers, Heist, InfamyPerk, InfamyPerks, RepeatBonus, Rewards,
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
    struct RotationData {
        index: usize,
        heist_preset: HeistPreset,
        strategy_preset: StrategyPreset,
        heist: Heist,
        rewards: Rewards,
        cum_rewards: Rewards,
        cum_time: f32,
    }

    fn rotation_data(rotation: &Rotation) -> impl Iterator<Item = RotationData> {
        rotation
            .to_repeats()
            .enumerate()
            .map(|(i, (heist_preset, strategy_preset, repeat_bonus))| {
                let heist = heist_preset.to_heist(strategy_preset.clone(), repeat_bonus);
                let multipliers = GeneralMultipliers {
                    group_boost: true,

                    cash_boost: true,
                    cash_gamepass: false,
                    cash_infamy_perks: InfamyPerks::Cash(InfamyPerk::new(8).unwrap()),

                    xp_boost: false,
                    xp_gamepass: false,
                    xp_infamy_perks: InfamyPerks::Xp(InfamyPerk::new(8).unwrap()),
                    xp_infamy_boost: false,
                };
                let rewards = heist.get_reward(&multipliers);
                RotationData {
                    index: i,
                    heist_preset,
                    strategy_preset,
                    heist,
                    rewards: rewards.clone(),
                    cum_rewards: rewards,
                    cum_time: 0.0,
                }
            })
            .scan(
                (Rewards { cash: 0, xp: 0 }, 0.0),
                |(reward_state, time_state), mut rotation_data| {
                    reward_state.cash += rotation_data.rewards.cash;
                    reward_state.xp += rotation_data.rewards.xp;

                    *time_state += rotation_data.heist.get_strategy().in_game_time;

                    rotation_data.cum_rewards = reward_state.clone();
                    rotation_data.cum_time = *time_state;

                    Some(rotation_data)
                },
            )
    }

    let (rotation, set_rotation) = signal(Rotation::default());
    let (store, _) = signal(PresetStore::generate_default_presets());

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
                <td>Cum. Time</td>
                <td>Stealth</td>
                <td>Repeat</td>
                <td>Base Cash</td>
                <td>Cash</td>
                <td>Cash / Min</td>
                <td>Cum. Cash</td>
                <td>Base XP</td>
                <td>XP</td>
                <td>XP / Min</td>
                <td>Cum. XP</td>
            </tr>
            {move || {
                let rotation = rotation();
                rotation_data(&rotation).map(|rotation_data| {
                    let strat = rotation_data.heist.get_strategy();
                    let repeat_percent = (rotation_data.heist.get_multipliers().repeat_bonus.get_bonus() * 100.0) as u32;

                    fn display_time(time: f32) -> String {
                        format!("{}:{:0>2}", (time / 60.0) as u32, time % 60.0)
                    }
                    view! {
                        <tr>
                            <SelectHeist
                                heist_preset=rotation_data.heist_preset
                                strategy_preset=rotation_data.strategy_preset
                                store
                                update=move |heist_preset, strategy_preset| set_rotation
                                    .update(|rot| rot.0[rotation_data.index] = (heist_preset, strategy_preset))
                            />
                            <td>{display_time(strat.in_game_time)}</td>
                            <td>{display_time(rotation_data.cum_time)}</td>
                            <td>{strat.stealth}</td>
                            <td>{repeat_percent}"%"</td>
                            <td>{strat.base_cash}</td>
                            <td>{rotation_data.rewards.cash}</td>
                            <td>{rotation_data.rewards.cash as f32 / strat.in_game_time}</td>
                            <td>{rotation_data.cum_rewards.cash}</td>
                            <td>{strat.base_xp}</td>
                            <td>{rotation_data.rewards.xp}</td>
                            <td>{rotation_data.rewards.xp as f32 / strat.in_game_time}</td>
                            <td>{rotation_data.cum_rewards.xp}</td>
                        </tr>
                    }
                }).collect::<Vec<_>>()}
            }
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
    view! {
        <div>
            <Calculator/>
        </div>
    }
}

pub fn main() {
    mount_to_body(|| view! { <App /> })
}
