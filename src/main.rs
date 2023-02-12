use yew::prelude::*;
use yew_router::prelude::*;
mod comps;
use comps::home::home;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => home()
    }
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
