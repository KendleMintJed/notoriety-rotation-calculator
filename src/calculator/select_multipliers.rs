use leptos::prelude::*;

use calculator::{GeneralMultipliers, InfamyPerk, InfamyPerks};

#[component]
pub fn SelectMultipliers(
    multipliers: RwSignal<GeneralMultipliers>,
    infamy_skills: RwSignal<InfamyPerks>,
) -> impl IntoView {
    view! {
        <div>
            <input type="checkbox"
                prop:checked=move || multipliers().group_boost
                on:input:target=move |ev| multipliers.update(|multipliers| multipliers.group_boost = ev.target().checked())
            />"Group Boost (15%)"<br/>
            <br/>
            <input type="checkbox"
                prop:checked=move || multipliers().cash_boost
                on:input:target=move |ev| multipliers.update(|multipliers| multipliers.cash_boost = ev.target().checked())
            />"Cash Boost (100%)"<br/>
            <input type="checkbox"
                prop:checked=move || multipliers().cash_gamepass
                on:input:target=move |ev| multipliers.update(|multipliers| multipliers.cash_gamepass = ev.target().checked())
            />"Double Cash Gamepass (100%)"<br/>
            <select
                prop:value=move || multipliers().cash_infamy_perks.get_level()
                on:change:target=move |ev| multipliers.update(
                    |multipliers| multipliers.cash_infamy_perks = InfamyPerks::Cash(InfamyPerk::new(
                        ev.target().value().parse::<u8>().unwrap()).unwrap()
                    )
                )
            >
                <option value=0>0</option>
                <option value=1>1</option>
                <option value=2>2</option>
                <option value=3>3</option>
                <option value=4>4</option>
                <option value=5>5</option>
                <option value=6>6</option>
                <option value=7>7</option>
                <option value=8>8</option>
            </select>"Infamy Cash Perks"<br/>
            <br/>
            <input type="checkbox"
                prop:checked=move || multipliers().xp_boost
                on:input:target=move |ev| multipliers.update(|multipliers| multipliers.xp_boost = ev.target().checked())
            />"XP Boost (100%)"<br/>
            <input type="checkbox"
                prop:checked=move || multipliers().xp_gamepass
                on:input:target=move |ev| multipliers.update(|multipliers| multipliers.xp_gamepass = ev.target().checked())
            />"Double XP Gamepass (100%)"<br/>
            <select
                prop:value=move || multipliers().xp_infamy_perks.get_level()
                on:change:target=move |ev| multipliers.update(
                    |multipliers| multipliers.xp_infamy_perks = InfamyPerks::SkillPoints(InfamyPerk::new(
                        ev.target().value().parse::<u8>().unwrap()).unwrap()
                    )
                )
            >
                <option value=0>0</option>
                <option value=1>1</option>
                <option value=2>2</option>
                <option value=3>3</option>
                <option value=4>4</option>
                <option value=5>5</option>
                <option value=6>6</option>
                <option value=7>7</option>
                <option value=8>8</option>
            </select>"Infamy XP Perks"<br/>
            <input type="checkbox"
                prop:checked=move || multipliers().xp_infamy_boost
                on:input:target=move |ev| multipliers.update(|multipliers| multipliers.xp_infamy_boost = ev.target().checked())
            />"Infamy Xp Boost (15%)"<br/>
            <br/>
            "Total Cash Boost: "{move || (multipliers().get_cash_boost()*100.0) as u32}%<br/>
            "Total XP Boost: "{move || (multipliers().get_xp_boost()*100.0) as u32}%<br/>
            <br/>
            <select
                prop:value=move || infamy_skills().get_level()
                on:change:target=move |ev| infamy_skills.set(InfamyPerks::SkillPoints(InfamyPerk::new(
                        ev.target().value().parse::<u8>().unwrap()).unwrap()
                    )
                )
            >
                <option value=0>0</option>
                <option value=1>1</option>
                <option value=2>2</option>
                <option value=3>3</option>
                <option value=4>4</option>
                <option value=5>5</option>
                <option value=6>6</option>
                <option value=7>7</option>
                <option value=8>8</option>
            </select>"Infamy Skill Perks"
        </div>
    }
}
