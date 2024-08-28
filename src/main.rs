use yew::prelude::*;

struct Model {
    value: i64,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model { value: 0 });

    html! {
        <div>
            <h1>{ "Rusty Cars Go Fast" }</h1>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
