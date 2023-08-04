use leptos::*;

struct TodoItem {
    title: String,
    done: bool,
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    use leptos::html::Input;
    use leptos::svg::view;
    let (count, set_count) = create_signal(cx, 0);
    let (todo, set_todo) = create_signal(cx, "".to_string());
    let (todos, set_todos) = create_signal::<Vec<String>>(cx, vec![]);
    let input_element: NodeRef<Input> = create_node_ref(cx);
    view! { cx,
        <input type="text"
                // here, we use the `value` *attribute* to set only
                // the initial value, letting the browser maintain
                // the state after that
                value=todo

                // store a reference to this input in `input_element`
                node_ref=input_element
            />
        <button
            on:click=move |_| {
                let value = input_element().expect("").value();
                set_todos.update(|n| n.push(value));
                set_todo("".to_string());
            }
        >
            "Click me: "
        </button>
        <ul>
            <For each=todos key=|tod| tod
            view=move |cx,tod| {
                view! { cx,
                    <li>
                        {todos}
                    </li>
                }
            }
            />

        </ul>

    }
}
