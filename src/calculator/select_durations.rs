use leptos::prelude::*;

use calculator::RealTimeDurations;

#[component]
pub fn SelectDurations(durations: RwSignal<RealTimeDurations>) -> impl IntoView {
    view! {
        <div>
            "Load heist: "<input type="number"
                prop:value=move || durations().load_heist
                on:input:target=move |ev| durations.update(|durations| {
                    if let Ok(value) = ev.target().value().parse() {
                        durations.load_heist = value;
                    }
                })
            /><br/>
            "Load lobby: "<input type="number"
                prop:value=move || durations().load_lobby
                on:input:target=move |ev| durations.update(|durations| {
                    if let Ok(value) = ev.target().value().parse() {
                        durations.load_lobby = value;
                    }
                })
            /><br/>
            "Host lobby: "<input type="number"
                prop:value=move || durations().host_lobby
                on:input:target=move |ev| durations.update(|durations| {
                    if let Ok(value) = ev.target().value().parse() {
                        durations.host_lobby = value;
                    }
                })
            /><br/>
            "Infamy: "<input type="number"
                prop:value=move || durations().infamy
                on:input:target=move |ev| durations.update(|durations| {
                    if let Ok(value) = ev.target().value().parse() {
                        durations.infamy = value;
                    }
                })
            /><br/>
            "Skill-break overhead: "<input type="number"
                prop:value=move || durations().skill_break_overhead
                on:input:target=move |ev| durations.update(|durations| {
                    if let Ok(value) = ev.target().value().parse() {
                        durations.skill_break_overhead = value;
                    }
                })
            /><br/>
            "Skill-break skills per second: "<input type="number"
                prop:value=move || durations().skill_break_skills_per_sec
                on:input:target=move |ev| durations.update(|durations| {
                    if let Ok(value) = ev.target().value().parse() {
                        durations.skill_break_skills_per_sec = value;
                    }
                })
            /><br/>
        </div>
    }
}
