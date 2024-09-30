use yew::prelude::*;
use yew_router::prelude::*;

mod key_input;
mod result_table;

use key_input::KeyInput;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <KeyInput /> },
        Route::NotFound => html! { <Redirect<Route> to={Route::Home}/> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}