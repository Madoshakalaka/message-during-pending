use std::future::Future;

use yew::lazy::declare_lazy_component;
use yew::prelude::*;
use yew::suspense::Suspension;

// ---------------------------------------------------------------------------
// A simple counter component — this is the one we'll load "lazily".
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CounterProps {
    pub label: AttrValue,
}

#[component]
fn Counter(props: &CounterProps) -> Html {
    let count = use_state(|| 0_i32);
    let onclick = {
        let count = count.clone();
        Callback::from(move |_| count.set(*count + 1))
    };

    html! {
        <div class="counter">
            <h3>{ &props.label }</h3>
            <p class="count">{ *count }</p>
            <button {onclick}>{ "+1" }</button>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Use the real declare_lazy_component!() macro, exactly as a user would.
// ---------------------------------------------------------------------------

declare_lazy_component!(Counter as LazyCounter in lazy_counter);

// ---------------------------------------------------------------------------
// A permanently-pending component so Suspense shows its fallback when the
// lazy component is toggled off (mirrors the original split-wasm example).
// ---------------------------------------------------------------------------

#[component]
fn Pending() -> HtmlResult {
    Err(Suspension::from_future(std::future::pending()).into())
}

// ---------------------------------------------------------------------------
// App: shows a normal counter and a lazy counter side-by-side.
// ---------------------------------------------------------------------------

#[component]
pub fn App() -> Html {
    let show_lazy = use_state(|| false);
    let toggle = {
        let show_lazy = show_lazy.clone();
        Callback::from(move |_: InputEvent| show_lazy.set(!*show_lazy))
    };

    html! {
        <main>
            <h1>{ "Lazy Component Message Bug Demo" }</h1>

            <section class="side-by-side">
                // --- Normal counter (works fine) ---
                <Counter label="Normal counter" />

                // --- Lazily-loaded counter (broken state updates) ---
                <div class="counter">
                    <h3>{ "Lazy counter" }</h3>
                    <label>
                        <input type="checkbox"
                            checked={*show_lazy}
                            oninput={toggle} />
                        { " Load lazy component" }
                    </label>
                    <Suspense fallback={html! { <p class="loading">{ "Loading…" }</p> }}>
                        if *show_lazy {
                            <LazyCounter label="Lazy counter (loaded)" />
                        } else {
                            <Pending />
                        }
                    </Suspense>
                </div>
            </section>

            <p class="explanation">
                { "The normal counter increments when you click +1. " }
                { "The lazy counter renders correctly but " }
                <strong>{ "clicking +1 does nothing" }</strong>
                { " — use_state re-render signals are silently lost because " }
                { "the inner scope was never mounted." }
            </p>
        </main>
    }
}
