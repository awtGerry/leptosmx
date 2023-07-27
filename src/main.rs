use leptos::*;

#[allow(non_snake_case)]

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Counter />
        // <Greet />
    }
}

#[component]
fn Counter(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            {count}
        </button>
    }
}

// #[component]
// fn Greet(cx: Scope) -> impl IntoView {
//     let (name, set_name) = create_signal(cx, "".to_string());
//     view! { cx,
//         <input type="text"
//             on:input=move |ev| {
//                 set_name(event_target_value(&ev));
//             }
//             prop:value={name}
//         />
//         <p>"Hello: " {name}</p>
//     }
// }
