mod key_input;
mod result_table;

use yew::prelude::*;
use yew::Renderer;
use crate::key_input::KeyInput;

#[function_component(Main)]
fn main_component() -> Html {

    html! {
        <div>
            <KeyInput />
        </div>
    }
}

fn main() {
    Renderer::<Main>::new().render();
}