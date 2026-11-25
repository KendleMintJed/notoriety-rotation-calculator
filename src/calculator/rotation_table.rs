use leptos::{either::EitherOf4, prelude::*};

use calculator::{
    RealTimeDurations, RealTimeEvents, Rotation,
    bonuses::{GeneralMultipliers, perks::SkillPerk},
    presets::PresetStore,
};

mod select_heist;
use select_heist::SelectHeist;

#[component]
pub fn RotationTable(
    rotation: ReadSignal<Rotation>,
    set_rotation: WriteSignal<Rotation>,
    multipliers: ReadSignal<GeneralMultipliers>,
    infamy_skills: ReadSignal<SkillPerk>,
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
                <td>{move || infamy_skills().get_bonus() as u8 + 1}</td>
            </tr>
            {move || {
                let rotation = rotation();
                let multipliers = multipliers();
                let infamy_skills = infamy_skills();
                let durations = RealTimeDurations::default();

                fn display_time(time: f32) -> String {
                    format!("{}:{:0>2}", (time / 60.0) as u32, (time % 60.0).round())
                }

                rotation
                .to_cum_real_time(&multipliers, &infamy_skills, &durations)
                .scan(0, |i, (cum_rta, event)| {
                    let index = *i;
                    if let RealTimeEvents::Heist(_) = event {
                        *i += 1;
                    }
                    Some((index, (cum_rta, event)))
                })
                .map(|(i, (cum_rta, event))| {
                    let rta = event.get_time(&durations);
                    match event {
                        RealTimeEvents::Heist(rotation_data) => {
                            let strat = rotation_data.heist.get_strategy();
                            let level = rotation_data.cum_level;
                            let repeat_percent = (rotation_data.heist.get_multipliers().repeat_bonus.get_bonus() * 100.0) as u32;
                            EitherOf4::A(view! {
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
                                        heist_preset=rotation_data.heist_preset
                                        strategy_preset=rotation_data.strategy_preset
                                        store
                                        update=move |heist_preset, strategy_preset| set_rotation
                                            .update(|rot| rot.0[i] = (heist_preset, strategy_preset))
                                    />
                                    <td>{display_time(rta)}</td>
                                    <td>{display_time(cum_rta)}</td>
                                    <td>lvl{level.get_level()}</td>
                                    <td>{level.get_skill_points() + infamy_skills.get_bonus() as u8}pts</td>
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
                            })
                        },
                        RealTimeEvents::Lobby => EitherOf4::B(view! {
                            <tr>
                                <td></td>
                                <td></td>
                                <td>Lobby</td>
                                <td></td>
                                <td>{display_time(rta)}</td>
                                <td>{display_time(cum_rta)}</td>
                            </tr>
                        }),
                        RealTimeEvents::SkillBreak(_) => EitherOf4::C(view! {
                            <tr>
                                <td></td>
                                <td></td>
                                <td>Skill Break</td>
                                <td></td>
                                <td>{display_time(rta)}</td>
                                <td>{display_time(cum_rta)}</td>
                            </tr>
                        }),
                        RealTimeEvents::Infamy => EitherOf4::D(view! {
                            <tr>
                                <td></td>
                                <td></td>
                                <td>Infamy</td>
                                <td></td>
                                <td>{display_time(rta)}</td>
                                <td>{display_time(cum_rta)}</td>
                            </tr>
                        }),
                    }
                }).collect::<Vec<_>>()}
            }
        </tbody></table>
    }
}
