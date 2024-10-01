use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, KeyboardEvent};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

use crate::result_table::{ResultTable, PickHistory};

const BASE_URL: &str = "http://localhost:8080";

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ApiPickHistory {
    time: String,
    pick_group: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ApiResponse {
    data: String,
    logs: Vec<ApiPickHistory>,
}

#[function_component(KeyInput)]
pub fn key_input() -> Html {
    let key_words = use_state(String::new);
    let current_pick = use_state(|| None::<String>);
    let pick_histories = use_state(Vec::new);
    let input_ref = use_node_ref();

    let on_text_input = {
        let key_words = key_words.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            key_words.set(input.value());
        })
    };

    let handle_click = {
        let key_words = key_words.clone();
        let current_pick = current_pick.clone();
        let pick_histories = pick_histories.clone();

        Callback::from(move |_| {
            let key_words = key_words.clone();
            let current_pick = current_pick.clone();
            let pick_histories = pick_histories.clone();

            spawn_local(async move {
                let payload = serde_json::json!({ "encryptCode": (*key_words).clone() });
                match Request::post(BASE_URL)
                    .json(&payload)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(response) => {
                        if let Ok(data) = response.json::<ApiResponse>().await {
                            current_pick.set(Some(data.data));
                            let converted_logs: Vec<PickHistory> = data.logs
                                .into_iter()
                                .map(|log| PickHistory {
                                    time: log.time,
                                    pick_group: log.pick_group,
                                })
                                .collect();
                            pick_histories.set(converted_logs);
                        }
                    }
                    Err(err) => {
                        log::error!("Error: {:?}", err);
                        current_pick.set(None);
                        pick_histories.set(vec![]);
                    }
                }
            });
        })
    };

    let on_key_up = {
        let handle_click = handle_click.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                handle_click.emit(());
            }
        })
    };

    use_effect_with_deps(
        move |input_ref| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                input.focus().unwrap();
            }
            || ()
        },
        input_ref.clone(),
    );

    html! {
        <div>
            <div class="background-container">
                <div class="selector-container">
                    <div class="input-container">
                        <div class="keyword-container">
                            <input
                                class="keyword-input"
                                oninput={on_text_input}
                                value={(*key_words).clone()}
                                ref={input_ref}
                                onkeyup={on_key_up}
                            />
                            <button class="confirm-button" onclick={Callback::from(move |_: MouseEvent| handle_click.emit(()))}>{"生成英雄"}</button>
                        </div>
                        if let Some(pick) = &*current_pick {
                            <div class="hint-container">
                                <div class="backdrop"></div>
                                <p>{pick}</p>
                            </div>
                        }
                        <ResultTable data={(*pick_histories).clone()} />
                    </div>
                </div>
            </div>
        </div>
    }
}