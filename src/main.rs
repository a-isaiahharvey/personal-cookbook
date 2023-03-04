mod components;
mod pages;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => pages::home::page(),
        Route::About => html! { <p>{ "About" }</p> },
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
