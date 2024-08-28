use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Rusty Cars Go Fast" }</h1>
            <HamburgerMenu />
            <p>{ "Welcome to our homepage!" }</p>
        </div>
    }
}

#[function_component(HamburgerMenu)]
fn hamburger_menu() -> Html {
    let menu_open = use_state(|| false);

    let toggle_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(!*menu_open))
    };

    html! {
        <div>
            <button class="hamburger-button" onclick={toggle_menu.clone()}>
                { if *menu_open { "✕" } else { "☰" } }
            </button>
            if *menu_open {
                <nav class="menu">
                    <ul>
                        <li><a href="#home">{"Home"}</a></li>
                        <li><a href="#about">{"About"}</a></li>
                        <li><a href="#services">{"Services"}</a></li>
                        <li><a href="#contact">{"Contact"}</a></li>
                    </ul>
                </nav>
            }
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
