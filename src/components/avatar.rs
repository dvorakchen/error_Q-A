use leptos::*;

#[component]
pub fn Avatar() -> impl IntoView {
    view! {
        <div class="avatar">
            <div class="w-12 rounded-full ring ring-white">
                <img alt="Avatar" src="/images/avatar.png"/>
            </div>
        </div>
    }
}
