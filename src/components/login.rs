use std::rc::Rc;

use gloo::utils::window;
use leptos::html::{Input, Section};
use leptos::*;
use web_sys::SubmitEvent;

use crate::http::{login, send_captcha, valid_captcha};

#[component]
pub fn Login() -> impl IntoView {
    let (show_article, set_show_article) = create_signal(false);
    let (error_captcha, set_error_captcha) = create_signal(false);

    let email_section: NodeRef<Section> = create_node_ref();
    let email_section_captcha = Rc::new(email_section);
    let email_section_back = Rc::clone(&email_section_captcha);

    let captcha_input: NodeRef<Input> = create_node_ref();

    let email_input = create_node_ref();
    let email_input_for_send = Rc::new(email_input);
    let email_input_for_valid = Rc::clone(&email_input_for_send);

    let captcha_action = create_action(move |input: &String| {
        let input = input.to_owned();
        let email_section_captcha = Rc::clone(&email_section_captcha);
        async move {
            send_captcha(&input).await;
            let email_section_element = email_section_captcha
                .get_untracked()
                .expect("email section not exist");

            _ = email_section_element.style("margin-left", "calc(-100% - .5rem)");
        }
    });
    let captcha_pending = captcha_action.pending();

    let captcha_valid_action = create_action(move |input: &(String, String)| {
        let (email, captcha) = input.clone();
        let set_error_captcha = set_error_captcha;
        async move {
            let res = valid_captcha(&email, &captcha).await;
            if res {
                login(&email);
                let location = window().location();
                location.set_href("/").expect("redirect fail");
            } else {
                set_error_captcha.set(true);
            }
        }
    });
    let captcha_valid_pending = captcha_valid_action.pending();

    let handle_send_captcha = move |ev: SubmitEvent| {
        ev.prevent_default();

        let email_element: HtmlElement<Input> =
            email_input_for_send.get().expect("email input not exist");
        let email_address = email_element.value();

        captcha_action.dispatch(email_address);
    };

    let handle_back = move |_ev| {
        let email_element = email_section_back.get().expect("email section not exist");
        _ = email_element.style("margin-left", "");
    };

    let handle_login = move |ev: SubmitEvent| {
        ev.prevent_default();

        let email_element: HtmlElement<Input> =
            email_input_for_valid.get().expect("email input not exist");
        let email_address = email_element.value();

        let captcha_input = captcha_input.get().expect("CAPTCHA input not exist");
        let captcha = captcha_input.value();

        captcha_valid_action.dispatch((email_address, captcha));
    };

    view! {
        <div class="h-screen w-full flex place-content-center flex-col">
            <div class="flex place-items-center flex-col space-y-4 -mt-4">
                <a href="/" class="btn btn-ghost text-primary text-2xl">
                    异常
                </a>
                <div class="card shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
                    <div class="card-body">
                        <main class="flex overflow-hidden px-2 gap-2">
                            <section
                                class="min-w-full transition-all duration-500"
                                node_ref=email_section
                            >
                                <form on:submit=handle_send_captcha>
                                    <div class="form-control">
                                        <label class="label" for="email">
                                            <span class="label-text">请输入有效邮箱</span>
                                        </label>
                                        <input
                                            node_ref=email_input
                                            id="email"
                                            type="email"
                                            placeholder="邮箱"
                                            class="input input-bordered"
                                            required
                                            oninvalid="setCustomValidity('请输入有效邮箱')"
                                            oninput="setCustomValidity('');"
                                        />
                                    </div>
                                    <div class="form-control mt-6">
                                        <button
                                            class="btn btn-primary"
                                            prop:disabled=captcha_pending
                                        >
                                            {move || {
                                                captcha_pending
                                                    .get()
                                                    .then(|| {
                                                        view! { <span class="loading loading-spinner"></span> }
                                                    })
                                            }}

                                            发送验证码
                                        </button>
                                    </div>
                                </form>
                            </section>
                            <section class="min-w-full">
                                <form on:submit=handle_login>
                                    <div class="form-control">
                                        <label class="label" for="captcha">
                                            <span class="label-text">验证码</span>
                                        </label>
                                        <input
                                            node_ref=captcha_input
                                            id="captcha"
                                            type="text"
                                            placeholder="请输入收到的验证码，当前固定为 1"
                                            class="input input-bordered"
                                            required
                                            oninvalid="setCustomValidity('请输入收到的验证码')"
                                            oninput="setCustomValidity('');"
                                        />
                                    </div>
                                    {move || {
                                        error_captcha
                                            .get()
                                            .then(|| {
                                                view! {
                                                    <div class="form-control mt-3 text-error">
                                                        "验证码错误"
                                                    </div>
                                                }
                                            })
                                    }}

                                    <div class="form-control mt-6">
                                        <div class="flex space-4 justify-between">
                                            <button type="button" class="btn" on:click=handle_back>
                                                返回
                                            </button>
                                            <button
                                                class="btn btn-primary"
                                                prop:disabled=captcha_valid_pending
                                            >
                                                {move || {
                                                    captcha_valid_pending
                                                        .get()
                                                        .then(|| {
                                                            view! { <span class="loading loading-spinner"></span> }
                                                        })
                                                }}

                                                登入
                                            </button>
                                        </div>

                                    </div>
                                </form>

                            </section>
                        </main>
                    </div>
                </div>
                <div class="flex flex-col">
                    <button
                        class="btn btn-link"
                        on:click=move |_ev| {
                            set_show_article
                                .update(|v| {
                                    *v = !*v;
                                })
                        }
                    >

                        "没有账号？不需要注册"
                    </button>
                    <div
                        class="grid transition-all duration-500 grid-rows-[0fr]"
                        style:grid-template-rows=move || {
                            if show_article.get() { "1fr" } else { "0fr" }
                        }
                    >

                        <div class="overflow-hidden">
                            <article class="prose text-primary text-center">
                                <p>
                                    "不需要注册账号，你只需要提供一个属于你的、常用的、有效邮箱即可"
                                </p>
                                <p>
                                    "这是个简单的应用，实在没有注册账号的必要。反正不管是提问还是回答，都只需要一个能够接收到通知的邮箱，那么为什么还需要什么账号密码呢"
                                </p>
                                <p>
                                    "所以你只需要提供一个你自己的邮箱，然后我们会发送验证码给你，你输入接收到的验证码，以证明这个邮箱是你的就可以了"
                                </p>
                                <p>"省的太麻烦"</p>
                            </article>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
