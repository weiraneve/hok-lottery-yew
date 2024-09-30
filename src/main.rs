use crate::app::App;

mod app;
mod pages;
mod routes;

fn main() {
    yew::Renderer::<App>::new().render();
}
