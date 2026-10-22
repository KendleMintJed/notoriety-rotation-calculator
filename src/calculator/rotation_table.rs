use leptos::prelude::*;

use ::calculator::{
    GeneralMultipliers, Heist, Rewards,
    data::{HeistPreset, PresetStore, Rotation, StrategyPreset},
};

mod select_heist;
use select_heist::SelectHeist;

struct RotationData {
    index: usize,
    heist_preset: HeistPreset,
    strategy_preset: StrategyPreset,
    heist: Heist,
    rewards: Rewards,
    cum_rewards: Rewards,
    cum_time: f32,
}

fn rotation_data(
    rotation: &Rotation,
    multipliers: &GeneralMultipliers,
) -> impl Iterator<Item = RotationData> {
    rotation
        .to_repeats()
        .enumerate()
        .map(|(i, (heist_preset, strategy_preset, repeat_bonus))| {
            let heist = heist_preset.to_heist(strategy_preset.clone(), repeat_bonus);
            let rewards = heist.get_reward(multipliers);
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

#[component]
pub fn RotationTable(
    rotation: ReadSignal<Rotation>,
    set_rotation: WriteSignal<Rotation>,
    multipliers: ReadSignal<GeneralMultipliers>,
    store: ReadSignal<PresetStore>,
) -> impl IntoView {
    view! {
        <table><tbody>
            <tr>
                <td>Edit</td>
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
            <tr><button
                on:click=move |_| {
                    let store = store();
                    let (default_heist, default_strat) = store.get_default();
                    let default = (default_heist.clone(), default_strat.clone());
                    set_rotation.update(|rotation| rotation.0.push(default));
                }
            >"+"
            </button></tr>
            {move || {
                let rotation = rotation();
                let multipliers = multipliers();
                rotation_data(&rotation, &multipliers).map(|rotation_data| {
                    let strat = rotation_data.heist.get_strategy();
                    let repeat_percent = (rotation_data.heist.get_multipliers().repeat_bonus.get_bonus() * 100.0) as u32;

                    fn display_time(time: f32) -> String {
                        format!("{}:{:0>2}", (time / 60.0) as u32, time % 60.0)
                    }
                    view! {
                        <tr>
                            <td>
                                <button
                                    on:click=move |_| set_rotation.update(|rotation| {rotation.0.remove(rotation_data.index);})
                                >x</button>
                                <button
                                    on:click=move |_| {
                                        let i = rotation_data.index;
                                        if i > 0 {set_rotation.update(|rotation| rotation.0.swap(i, i-1))};
                                    }
                                >^</button>
                            </td>
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
        </tbody></table>
    }
}
