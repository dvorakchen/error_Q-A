use leptos::*;

use crate::{
    data::{PaginateOrder, Pagination},
    http::get_hot_post_list,
};

use super::post_list::PostList;

#[component]
pub fn MainContent() -> impl IntoView {
    view! {
        <div class="grid grid-cols-[0fr_3fr_0fr] md:grid-cols-[1fr_3fr_1fr]">
            <header class="col-start-1 col-end-4 self-center justify-self-center">
                <Summary/>
            </header>
            <div class="ml-auto w-0 md:w-40 overflow-hidden">
                <Lanugages/>
            </div>
            <main class="w-auto p-4 space-y-4">
                <h1 class="text-xl">"最新提问:"</h1>
                <PostList pagination=Pagination::new(0, 10, PaginateOrder::DateTimeDescent)/>
            </main>
            <div class="mr-auto w-0 md:w-40 overflow-hidden">
                <HotNetworkQuestions/>
            </div>
        </div>
    }
}

#[component]
fn Summary() -> impl IntoView {
    view! {
        <div class="flex flex-col md:flex-row place-content-center place-items-center space-x-4
        p-4">
            <div class="flex place-items-center w-40 h-40">
                <img src="/images/rustacean-flat-happy.png" alt="ferris happy"/>
            </div>
            <article class="prose mt-4 pt-4">
                <h1 class="text-primary">
                    "使用 "
                    <a
                        href="https://leptos.dev/"
                        target="_blank"
                        class="text-primary no-underline font-bold hover:underline"
                    >
                        "Leptos"
                    </a> ": "
                    <a
                        href="https://www.rust-lang.org/"
                        target="_blank"
                        class="text-primary no-underline font-bold hover:underline"
                    >
                        "Rust"
                    </a> " + "
                    <a
                        href="https://daisyui.com/"
                        target="_blank"
                        class="text-primary no-underline font-bold hover:underline"
                    >
                        "DaisyUI"
                    </a> " + "
                    <a
                        href="https://tailwindcss.com/"
                        target="_blank"
                        class="text-primary no-underline font-bold hover:underline"
                    >
                        "Tailwindcss"
                    </a> " 制作"
                </h1>
                <h2 class="text-primary">"不具备任何功能"</h2>
                <h2 class="text-primary">"仅展示个人学习成果使用"</h2>
            </article>
        </div>
    }
}

#[component]
fn Lanugages() -> impl IntoView {
    view! {
        <div class="py-4 px-2 bg-base-200/30 rounded-md backdrop-blur-sm
        bg-gradient-to-bl from-primary to-secondary">
            <ul class="space-y-2">
                <li>
                    <a href="/search?lang=C">"C"</a>
                </li>
                <li>
                    <a href="/search?lang=Cplusplus">"C++"</a>
                </li>
                <li>
                    <a href="/search?lang=Csharp">"C#"</a>
                </li>
                <li>
                    <a href="/search?lang=Python">"Python"</a>
                </li>
                <li>
                    <a href="/search?lang=Javascript">"Javascript"</a>
                </li>
            </ul>
            <div class="my-2">
                <a href="javascript:;" class="btn btn-link">
                    more languages
                </a>
            </div>
        </div>
    }
}

#[component]
fn HotNetworkQuestions() -> impl IntoView {
    let hot_posts = create_resource(|| {}, |_| async move { get_hot_post_list().await });

    view! {
        <div class="py-4 px-2 bg-base-200/30 rounded-md backdrop-blur-sm
        bg-gradient-to-br from-primary to-secondary">
            <div class="mb-4 bg-primary backdrop-blur rounded shadow-lg shadow-primary">
                "热门提问："
            </div>
            <ul class="overflow-hidden w-auto space-y-2">
                <Suspense fallback=SkeletonHotPost>

                    {move || {
                        match hot_posts.get() {
                            None => view! { "" }.into_view(),
                            Some(list) => {
                                list.into_iter()
                                    .map(|post| {
                                        view! {
                                            <a
                                                href=format!("/posts/{}/{}", post.id, post.title)
                                                class="tooltip whitespace-nowrap"
                                                data-tip=post.title.clone()
                                            >
                                                {post.title}
                                            </a>
                                        }
                                    })
                                    .collect_view()
                            }
                        }
                    }}

                </Suspense>
            </ul>
        </div>
    }
}

#[component]
fn SkeletonHotPost() -> impl IntoView {
    view! { (0..5).into_iter().map(|_| view! { <div class="skeleton w-24 h-6"></div> }).collect_view() }
}
