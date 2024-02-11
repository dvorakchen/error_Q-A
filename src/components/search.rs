use leptos::*;
use leptos_router::use_query_map;

use crate::components::Pagination;
use crate::{
    data::{PaginateOrder, Pagination},
    http::get_post_list,
};

#[component]
pub fn Search() -> impl IntoView {
    let query = use_query_map();

    let q = move || {
        query.with_untracked(|query| query.get("q").cloned().unwrap_or_default())
    };
    // let search_key = q();
    // logging::log!("p: {}", search_key);

    let mut search = Pagination::new(1, 10, PaginateOrder::DateTimeDescent);
    search.q = Some(q());

    let index = create_rw_signal(1);

    let posts = create_resource(
        move || index.get(),
        move |index| {
            let mut search = search.clone();
            search.index = index;
            async move { get_post_list(search).await }
        },
    );

    view! {
        <div class="p-4 space-y-4">
        <div>
            <span class="text-3xl font-bold">"搜索："{move || {
                q()
            }}</span>
        </div>
            <ul class="space-y-4">
                <Suspense fallback=SkeletonPosts>

                    {move || {
                        match posts.get() {
                            None => view! { <li>"没有找到相关的提问"</li> }.into_view(),
                            Some(posts) => {
                                posts
                                    .into_iter()
                                    .map(|post| {
                                        view! {
                                            <li>
                                                <a
                                                    href=format!("/posts/{}/{}", post.id, post.title)
                                                    class="link link-hover"
                                                >
                                                    {post.title}
                                                </a>
                                            </li>
                                        }
                                    })
                                    .collect_view()
                            }
                        }
                    }}

                </Suspense>
            </ul>
            <Pagination index=index/>
        </div>
    }
}

#[component]
fn SkeletonPosts() -> impl IntoView {
    view! {
        {(0..5)
            .into_iter()
            .map(|_| {
                view! {
                    <li>
                        <div class="skeleton h-6 w-[300px]"></div>
                    </li>
                }
            })
            .collect_view()}
    }
}
