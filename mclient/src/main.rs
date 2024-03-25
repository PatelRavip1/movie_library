use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
use pages::home::HomePage;

#[derive(Routable, Clone, Debug, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage/> },
    }
}
#[function_component]
fn App() -> Html {
    html! {
        <>
            <div class="top-bar">
                <div class="logo-container">
                    <h1 >{"MyMovies"}</h1>
                </div>
                <div class="nav-container">
                    <a href="/">{"Home"}</a>
                    <a href="/add" >{"Add Movie"}</a>
                </div>
            </div>
            <BrowserRouter>
                <Switch<Route> render={Callback::from(|route: Route| switch(route))} />
            </BrowserRouter>

        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
