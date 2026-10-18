use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    view! {
        <div>
            <h1>"Welcome to Leptos"</h1>
            <p>"Your static site is ready!"</p>
            <button
                on:click = move |_| {
                    set_count(calculator::add(count(), 1))
                }
            >
                "Click me: "
                {count}
            </button>
        </div>
    }
}

pub fn main() {
    mount_to_body(|| view! { <App /> })
}
