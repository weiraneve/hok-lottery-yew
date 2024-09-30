mod key_input;
mod result_table;

use yew::prelude::*;
use yew::Renderer;
use crate::key_input::KeyInput;
use crate::result_table::{ResultTable, PickHistory};

#[function_component(Main)]
fn main_component() -> Html {
    let pick_histories: Vec<PickHistory> = vec![];

    html! {
        <div>
            <h1>{ "Main Component" }</h1>
            <KeyInput />
            <ResultTable data={pick_histories} />
        </div>
    }
}

fn main() {
    Renderer::<Main>::new().render();
}