use leptos::*;

const MAX_PAGE: usize = 4;
const MIN_PAGE: usize = 1;

#[component]
pub fn Pagination(index: RwSignal<usize>) -> impl IntoView {
    let handle_change_page = move |new_index: usize| {
        index.set(new_index);
    };

    let handle_left_arrow = move |_| {
        index.update(|v| *v = *v - 1);
    };

    let handle_right_arrow = move |_| {
        index.update(|v| *v = *v + 1);
    };

    view! {
        <div class="join mt-8">
            {move || {
                (index.get() > MIN_PAGE)
                    .then(|| {
                        view! {
                            <button class="join-item btn" on:click=handle_left_arrow>
                                "«"
                            </button>
                        }
                    })
            }}
            {move || {
                (MIN_PAGE..=MAX_PAGE)
                    .into_iter()
                    .map(|i| {
                        let current = i == index.get();
                        view! {
                            <input
                                class="join-item btn btn-square"
                                type="radio"
                                name="page"
                                aria-label=i
                                prop:checked=current
                                prop:disabled=current
                                on:click=move |_| {
                                    handle_change_page(i);
                                }
                            />
                        }
                    })
                    .collect_view()
            }}
            {move || {
                (index.get() < MAX_PAGE)
                    .then(|| {
                        view! {
                            <button class="join-item btn" on:click=handle_right_arrow>
                                "»"
                            </button>
                        }
                    })
            }}

        </div>
    }
}
