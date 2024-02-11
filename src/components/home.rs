use leptos::*;
use leptos_router::*;

use crate::components::Avatar;
use crate::http::{get_logged_email, logout};

mod main_content;
mod post_list;
pub use main_content::MainContent;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col justify-between">
            <Navbar/>
            <div class="mb-auto">
                <Outlet/>
            </div>
            <Footer/>
        </div>
    }
}

#[component]
fn Navbar() -> impl IntoView {
    view! {
        <nav class="navbar bg-base-100 shadow-lg justify-center gap-1">
            <a href="/" class="btn btn-ghost text-xl">
                异常
            </a>
            <div class="flex-none gap-2">
                <div class="form-control">
                    <Form method="get" action="/search">
                        <input
                            type="search"
                            name="q"
                            placeholder="搜索"
                            class="input input-bordered w-60
                            focus:w-96
                            transition-[width] duration-300"
                        />
                    </Form>

                </div>
                <Logged_in_or/>
            </div>
        </nav>
    }
}

/// this is a component that used to switch the login button and avatar
#[component]
fn Logged_in_or() -> impl IntoView {
    let logged_in = get_logged_email();

    let handle_logout = move |_| {
        logout();
        window()
            .location()
            .set_href("/")
            .expect("direct to homepage fail");
    };

    view! {
        <Show
            when=move || { logged_in.is_some() }
            fallback=|| {
                view! {
                    <A href="login" class="btn">
                        login
                    </A>
                }
            }
        >

            <div class="dropdown">
                <div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar">
                    <Avatar/>
                </div>
                <ul
                    tabindex="0"
                    class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52"
                >
                    <li>
                        <button on:click=handle_logout>"登出"</button>
                    </li>
                </ul>
            </div>

        </Show>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="footer items-center p-4 bg-neutral text-neutral-content">
            <aside class="items-center grid-flow-col">
                <span class="w-9">
                    <img src="/images/rustacean-flat-happy.png" alt="cute Ferris"/>
                </span>
                <p>"Copyright © 2024 - All right reserved"</p>
            </aside>
            <address class="justify-self-end flex space-x-2">
                <a href="https://dvorak.aiursoft.cn/" target="_blank" class="mr-4">
                    "Created by Dvorak"
                </a>
                <a href="https://github.com/dvorakchen" target="_blank">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        fill="currentColor"
                        class="bi bi-github"
                        viewBox="0 0 16 16"
                    >
                        <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27s1.36.09 2 .27c1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8"></path>
                    </svg>
                </a>
                <a href="mailto:dvorakchen@outlook.com">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        fill="currentColor"
                        class="bi bi-envelope"
                        viewBox="0 0 16 16"
                    >
                        <path d="M0 4a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2zm2-1a1 1 0 0 0-1 1v.217l7 4.2 7-4.2V4a1 1 0 0 0-1-1zm13 2.383-4.708 2.825L15 11.105zm-.034 6.876-5.64-3.471L8 9.583l-1.326-.795-5.64 3.47A1 1 0 0 0 2 13h12a1 1 0 0 0 .966-.741M1 11.105l4.708-2.897L1 5.383z"></path>
                    </svg>
                </a>
            </address>
        </footer>
    }
}
