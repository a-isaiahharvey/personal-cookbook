mod components;
mod pages;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/personal-cookbook/")]
    Home,
    #[not_found]
    #[at("/personal-cookbook/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => pages::home::page(),
        Route::NotFound => html! { <p>{ "Not Found" }</p> },
    }
}

/// main root
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

/// entry point
fn main() {
    yew::Renderer::<App>::new().render();
}
