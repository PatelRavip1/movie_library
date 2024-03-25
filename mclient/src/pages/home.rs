use yew::prelude::*;

#[function_component(HomePage)]
pub fn home() -> Html {
    let movie_items: Vec<Html> = (0..10)
        .map(|_| {
            html! {
                <div class="movie-item">
                    // <img class="movie-image" src="assets/logo.png" alt="Movie Title" />
                    <div class="movie-title">{ "Title" }</div>
                    <div class="movie-title">{ "ReleaseYear" }</div>
                    </div>
            }
        })
        .collect();

    html! {
        <>
            <div class="main-content">
                <div class="content-area">
                    <div class="movie-container">
                        { for movie_items }
                    </div>
                </div>
            </div>
        </>
    }
}
