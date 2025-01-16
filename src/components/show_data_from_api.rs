use leptos::prelude::*;

use crate::api::say_hello::say_hello;

#[component]
pub fn ShowDataFromApi() -> impl IntoView {
    let value = RwSignal::new("".to_string());
    let counter = RwSignal::new(0);

    let on_click = move |_| {
        leptos::task::spawn_local(async move {
            let api_said = say_hello(counter.get()).await.unwrap();
            value.set(api_said);
            counter.update(|v| *v += 1);
        });
    };

    view! {
        <div>
            <button on:click=on_click>"What does the API say?"</button>
            <p>{value}</p>
        </div>
    }
}
