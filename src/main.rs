use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (get_count, set_count) = create_signal(0);

    view! {
        <button on:click=move |_| {
            set_count(get_count() + 3);
        }>

            "Click me: " {move || get_count()}
        </button>
    }
}
