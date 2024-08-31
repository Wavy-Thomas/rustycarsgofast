use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use serde::Deserialize;
use web_sys::{HtmlSelectElement, Event};
use wasm_bindgen::JsCast;

#[derive(Deserialize, Clone)]
struct FileList {
    files: Vec<String>,
}

async fn fetch_csv_list() -> Result<FileList, Box<dyn std::error::Error>> {
    let url = "http://127.0.0.1:3030/list-csv-files"; // API endpoint to fetch the list of CSV files
    let response = Request::get(url).send().await?;
    let files = response.json::<FileList>().await?;
    Ok(files)
}

async fn fetch_csv_data(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://rustycarsgofast.s3.amazonaws.com/{}", file_name);
    let response = Request::get(&url).send().await?;
    let csv_content = response.text().await?;
    Ok(csv_content)
}

#[function_component(App)]
fn app() -> Html {
    let csv_list = use_state(|| None);
    let selected_file = use_state(|| None);
    let csv_data = use_state(|| None);

    {
        let csv_list = csv_list.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                match fetch_csv_list().await {
                    Ok(files) => csv_list.set(Some(files.files)),
                    Err(err) => eprintln!("Error fetching CSV list: {}", err),
                }
            });
            || ()
        }, ());
    }

    let on_file_change = {
        let selected_file = selected_file.clone();
        let csv_data = csv_data.clone();  // Clone here to capture reference in the closure
        Callback::from(move |event: Event| {
            let input = event.target().unwrap().dyn_into::<HtmlSelectElement>().unwrap();
            let file_name = input.value();
            selected_file.set(Some(file_name.clone()));
            let csv_data = csv_data.clone();  // Clone here to move the reference into async block
            spawn_local(async move {
                match fetch_csv_data(&file_name).await {
                    Ok(data) => csv_data.set(Some(data)),
                    Err(err) => eprintln!("Error fetching CSV data: {}", err),
                }
            });
        })
    };

    html! {
        <div>
            <h1>{ "CSV Data" }</h1>
            <select onchange={on_file_change}>
                <option value="">{ "Select a file" }</option>
                {
                    if let Some(files) = &*csv_list {
                        files.iter().map(|file| {
                            html! {
                                <option value={file.clone()}>{ file }</option>
                            }
                        }).collect::<Html>()
                    } else {
                        html! { <option disabled={true}>{ "Loading files..." }</option> }
                    }
                }
            </select>
            if let Some(ref data) = *csv_data {
                <pre>{ data }</pre>
            } else {
                <p>{ "No file selected." }</p>
            }
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
