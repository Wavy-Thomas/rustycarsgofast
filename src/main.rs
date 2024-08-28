use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

async fn fetch_csv_data() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://rustycarsgofast.s3.amazonaws.com/csv_files/data.csv";
    let response = Request::get(url).send().await?;
    let csv_content = response.text().await?;
    Ok(csv_content)
}

#[function_component(App)]
fn app() -> Html {
    let csv_data = use_state(|| None);

    {
        let csv_data = csv_data.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                match fetch_csv_data().await {
                    Ok(data) => csv_data.set(Some(data)),
                    Err(err) => eprintln!("Error fetching CSV: {}", err),
                }
            });
            || ()
        }, ());
    }

    html! {
        <div>
            <h1>{ "CSV Data" }</h1>
            if let Some(ref data) = *csv_data {
                <pre>{ data }</pre>
            } else {
                <p>{ "Loading..." }</p>
            }
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
