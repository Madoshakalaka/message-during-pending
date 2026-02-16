use std::future::{pending, Future};

use yew::lazy::declare_lazy_component;
use yew::prelude::*;
use yew::suspense::Suspension;

// ---------------------------------------------------------------------------
// A counter component — the one we'll load lazily.
// Uses use_state, which triggers re-renders via the scope it was created with.
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
// declare_lazy_component!() — the real macro from the PR, same pattern as
// examples/split-wasm/src/yew.rs line 24.
// ---------------------------------------------------------------------------

declare_lazy_component!(Counter as LazyCounter in lazy_counter);

// ---------------------------------------------------------------------------
// A permanently-pending component so Suspense keeps showing its fallback
// when the lazy component hasn't been toggled on yet.
// (Same pattern as examples/split-wasm/src/yew.rs line 26-29.)
// ---------------------------------------------------------------------------

#[component]
fn Pending() -> HtmlResult {
    Err(Suspension::from_future(pending()).into())
}

// ---------------------------------------------------------------------------
// App — normal counter next to a lazy counter to contrast behaviour.
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
            <h1>{ "Lazy Component Bug Demo" }</h1>

            <section class="side-by-side">
                // Normal counter — works perfectly.
                <Counter label="Normal counter" />

                // Lazy counter — initial render looks fine, but +1 button
                // silently does nothing because use_state's re-render signal
                // targets the unmounted inner_scope.
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
                { "Click +1 on both counters. The normal counter increments. " }
                { "The lazy counter " }
                <strong>{ "stays stuck at 0" }</strong>
                { " — use_state re-render signals are silently lost." }
            </p>
        </main>
    }
}
