use leptos::*;

use crate::data::{Pagination, PostThumbnail};
use crate::http::get_post_list;

#[component]
pub fn PostList(pagination: Pagination) -> impl IntoView {
    let p = pagination.clone();
    let posts = create_resource(
        || {},
        move |_| {
            let p = p.clone();
            async move { get_post_list(p).await }
        },
    );

    view! {
        <ul class="space-y-4">
            <Suspense fallback=SkeletonPosts>

                {move || {
                    match posts.get() {
                        None => view! { "" }.into_view(),
                        Some(list) => {
                            list.into_iter()
                                .map(|post| view! { <PostThumbnail thumbnail=post/> })
                                .collect_view()
                        }
                    }
                }}

            </Suspense>
        </ul>
    }
}

#[component]
fn SkeletonPosts() -> impl IntoView {
    view! {
        {(0..=5)
            .into_iter()
            .map(|_| {
                view! {
                    <div class="flex flex-col border-2 border-transparent
                    px-4 py-2 space-y-2 h-20">
                        <div class="skeleton h-6 w-48"></div>
                        <span class="skeleton h-6 w-24"></span>
                    </div>
                }
            })
            .collect_view()}
    }
}

#[component]
fn PostThumbnail(thumbnail: PostThumbnail) -> impl IntoView {
    view! {
        <div class="flex flex-col border-2 border-transparent
        px-4 py-2 space-y-2
        hover:border-primary hover:rounded hover:shadow">
            <div class="space-x-4">
                <span
                    class="tooltip"
                    data-tip=if thumbnail.solved { "已解决" } else { "待解决" }
                >
                    {if thumbnail.solved { "🥳" } else { "🥶" }}
                </span>
                <a
                    href=format!("/posts/{}/{}", thumbnail.id, thumbnail.title)
                    target="_blank"
                    class="hover:underline"
                >
                    {thumbnail.title}
                </a>
            </div>
            <div class="text-sm">{thumbnail.post_date_time}</div>
        </div>
    }
}
