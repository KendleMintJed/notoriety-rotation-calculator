use leptos::prelude::*;

mod calculator;
use crate::calculator::Calculator;

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
