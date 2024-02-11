use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{Home, Login, NotFound, Search, MainContent, Post};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes>
                <Route path="/" view=move || view! { <Home/> }>
                    <Route path="" view=MainContent/>
                    <Route path="search" view=Search/>
                    <Route path="posts/:id/:title" view=Post/>
                </Route>
                <Route path="login" view=Login/>
                <Route path="/*any" view=NotFound/>
            </Routes>
        </Router>
    }
}
