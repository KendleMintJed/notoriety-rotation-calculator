use leptos::prelude::*;

use calculator::{
    Level, RealTimeDurations, Rotation, bonuses::GeneralMultipliers, presets::PresetStore,
};

mod select_heist;
use select_heist::SelectHeist;

#[component]
pub fn RotationTable(
    rotation: ReadSignal<Rotation>,
    set_rotation: WriteSignal<Rotation>,
    multipliers: ReadSignal<GeneralMultipliers>,
    durations: ReadSignal<RealTimeDurations>,
    store: ReadSignal<PresetStore>,
) -> impl IntoView {
    view! {
        <table><tbody>
            <tr>
                <td>Edit</td>
                <td>#</td>
                <td>Heist</td>
                <td>Strategy</td>
                <td>RTA</td>
                <td>Cum. RTA</td>
                <td>Level</td>
                <td>Skills</td>
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
            <tr>
                <td><button
                    on:click=move |_| {
                        let store = store();
                        let (default_heist, default_strat) = store.get_default();
                        let default = (default_heist.clone(), default_strat.clone());
                        set_rotation.update(|rotation| rotation.0.push(default));
                    }
                >"+"</button></td>
                <td>0.</td>
                <td>Pre-Heist</td>
                <td></td>
                <td></td>
                <td></td>
                <td>lvl1</td>
                <td>{move || multipliers().skill_perks.get_bonus() as u8 + 1}</td>
            </tr>
            {move || {
                let rotation = rotation();
                let multipliers = multipliers();
                let durations = durations();

                fn display_time(time: f32) -> String {
                    format!("{}:{:0>2}", (time / 60.0) as u32, (time % 60.0).round())
                }

                rotation
                .calculate(&multipliers, &durations)
                .enumerate()
                .map(|(i, data)| {
                    let repeat_percent = (data.heist.get_multipliers().repeat_bonus.get_bonus() * 100.0) as u32;
                    let level = Level::from_xp(data.cum_rewards.xp);
                    let strat = &data.heist.get_strategy_preset().strategy;
                    view! {
                        <tr>
                            <td>
                                <button
                                    on:click=move |_| set_rotation.update(|rotation| {rotation.0.remove(i);})
                                >x</button>
                                <button
                                    on:click=move |_| {
                                        if i > 0 {set_rotation.update(|rotation| rotation.0.swap(i, i-1))};
                                    }
                                >^</button>
                            </td>
                            <td>{i + 1}.</td>
                            <SelectHeist
                                heist_preset=data.heist.get_heist_preset().clone()
                                strategy_preset=data.heist.get_strategy_preset().clone()
                                store
                                update=move |heist_preset, strategy_preset| set_rotation
                                    .update(|rot| rot.0[i] = (heist_preset, strategy_preset))
                            />
                            <td>{display_time(data.real_time.time())}</td>
                            <td>{display_time(data.cum_real_time)}</td>
                            <td>lvl{level.get_level()}</td>
                            <td>{level.get_skill_points() + multipliers.skill_perks.get_bonus() as u8}pts</td>
                            <td>{display_time(strat.in_game_time)}</td>
                            <td>{display_time(data.cum_in_game_time)}</td>
                            <td>{strat.stealth}</td>
                            <td>{repeat_percent}"%"</td>
                            <td>{strat.base_cash}</td>
                            <td>{data.rewards.cash}</td>
                            <td>{data.rewards.cash as f32 / strat.in_game_time}</td>
                            <td>{data.cum_rewards.cash}</td>
                            <td>{strat.base_xp}</td>
                            <td>{data.rewards.xp}</td>
                            <td>{data.rewards.xp as f32 / strat.in_game_time}</td>
                            <td>{data.cum_rewards.xp}</td>
                        </tr>
                    }
                }).collect::<Vec<_>>()}
            }
        </tbody></table>
    }
}
