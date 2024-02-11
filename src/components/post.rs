use leptos::*;
use leptos_router::*;

use crate::data::{Comment, Post};
use crate::http::{get_comments, get_post};

#[derive(Params, PartialEq, Eq, Debug)]
struct PostParams {
    id: Option<String>,
    title: Option<String>,
}

#[component]
pub fn Post() -> impl IntoView {
    let params = use_params_map();
    let id = params
        .get_untracked()
        .get("id")
        .expect("cannot get id")
        .clone();

    let post = create_resource(
        || {},
        move |_| {
            let id = id.clone();
            async move { get_post(id).await }
        },
    );

    view! {
        <div class="p-4">
            <main class="flex flex-col items-center space-x-4">
                <Suspense fallback=SkeletonPost>
                    {move || {
                        if let Some(post) = post.get() {
                            let id_for_comment = post.id.clone();
                            view! {
                                <PostDetail post=post/>
                                <Comments post_id=id_for_comment/>
                            }
                                .into_view()
                        } else {
                            view! { "POST NOT EXIST" }.into_view()
                        }
                    }}

                </Suspense>
            </main>
        </div>
    }
}

#[component]
fn PostDetail(post: Post) -> impl IntoView {
    view! {
        <article class="prose">
            <h1>{post.title}</h1>
            <div class="space-x-4">
                <span>"作者：" {post.author}</span>
                <span>"发布时间：" {post.post_date_time}</span>
            </div>
            <p>{post.content}</p>
        </article>
    }
}

#[component]
fn Comments(post_id: String) -> impl IntoView {
    let post_id_for_comment = post_id.clone();
    let comments = create_resource(
        || {},
        move |_| {
            let post_id = post_id_for_comment.clone();
            async move { get_comments(post_id).await }
        },
    );

    view! {
        <div class="flex flex-col w-full max-w-[65ch] mt-4">
            <div class="divider divider-primary divider-start">"回答："</div>
            <Suspense>
                {comments
                    .with(|comments| {
                        if let Some(comments) = comments {
                            comments
                                .into_iter()
                                .map(|comment| {
                                    view! { <CommentItem comment=comment.clone()/> }
                                })
                                .collect_view()
                                .into_view()
                        } else {
                            view! { "" }.into_view()
                        }
                    })}

            </Suspense>
        </div>
    }
}

#[component]
fn CommentItem(comment: Comment) -> impl IntoView {
    view! {
        <div class="border border-primary rounded-xl px-4 py-2">
            <div class="text-primary">{comment.author} "："</div>
            <div class="whitespace-pre-line">{comment.content}</div>
            <div class="text-sm mt-4">{comment.post_date_time}</div>
        </div>
    }
}

#[component]
fn SkeletonPost() -> impl IntoView {
    view! {
        <article class="w-full max-w-[48rem] space-y-4">
            <h1 class="skeleton h-12"></h1>
            <div class="space-x-4 flex">
                <div class="skeleton h-6 w-12"></div>
                <div class="skeleton h-6 w-12"></div>
            </div>
            <section class="w-full space-y-4">
                <h2 class="skeleton w-24 h-12"></h2>
                <p class="skeleton w-full h-48"></p>
            </section>
            <section class="w-full space-y-4">
                <h2 class="skeleton w-24 h-12"></h2>
                <p class="skeleton w-full h-48"></p>
            </section>
        </article>
    }
}
