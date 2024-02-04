use leptos::*;
use web_sys::KeyboardEvent;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Todo {
    id: usize,
    content: RwSignal<String>,
    is_done: RwSignal<bool>,
}

#[component]
fn TodoItem<F: Fn(usize) + 'static, T: Fn(usize) + 'static>(
    todo: Todo,
    on_change: F,
    on_delete: T,
) -> impl IntoView {
    let (is_editable, set_is_editable) = create_signal(false);
    let input_element: NodeRef<html::Input> = create_node_ref();

    let handle_edit = move |_| {
        set_is_editable(true);
        input_element().expect("<input> to exist").focus().unwrap();
    };

    let handle_keydown = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" {
            let el = input_element().unwrap();
            todo.content.set(el.value());
            set_is_editable(false);
            el.blur().unwrap();
        }
    };

    view! {
        <li class="border-t py-2 px-4 flex justify-between items-center">
            <input
                type="checkbox"
                prop:checked=move || todo.is_done.get()
                on:change=move |_| on_change(todo.id)
            />

            <input
                type="text"
                // TODO: ロジックが同じなので処理をまとめたい
                class=("line-through", move || todo.is_done.get())
                class=("text-gray-400", move || todo.is_done.get())
                value=todo.content.get()
                readonly=move || !is_editable()
                node_ref=input_element
                on:click=handle_edit
                on:keydown=handle_keydown
            />

            <button class="px-2" on:click=move |_| on_delete(todo.id)>
                x
            </button>
        </li>
    }
}

#[component]
fn App() -> impl IntoView {
    let (todos, set_todos) = create_signal::<Vec<Todo>>(vec![]);
    let todos_count = move || todos().len();
    let (item, setItem) = create_signal("".to_string());

    let handle_input = move |ev| setItem(event_target_value(&ev));

    let handle_submit = move |_| {
        if item().is_empty() {
            return;
        }
        let todo = Todo {
            id: todos().len() + 1,
            content: create_rw_signal(item()),
            is_done: create_rw_signal(false),
        };
        set_todos.update(|todos| todos.push(todo));
        setItem("".into());
    };

    let handle_change = move |id: usize| {
        set_todos.update(|todos| {
            if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
                todo.is_done.set(!todo.is_done.get());
            }
        });
    };

    let handle_remove = move |id: usize| {
        let todos = todos
            .get()
            .into_iter()
            .filter(|todo| todo.id != id)
            .collect();
        set_todos(todos);
    };

    view! {
        <div class="h-screen flex justify-center items-center flex-col">
            <main class="max-w-3xl text-center">
                <div class="border rounded">
                    <h1 class="p-2">
                        You have {todos_count}
                        {move || if todos_count() < 2 { " todo" } else { " todos" }}
                    </h1>
                    <ul>
                        <For
                            each=todos
                            key=|todo| todo.id
                            children=move |todo: Todo| {
                                view! {
                                    <TodoItem
                                        todo=todo
                                        on_change=handle_change
                                        on_delete=handle_remove
                                    />
                                }
                            }
                        />

                    </ul>
                    <div class="flex gap-2 p-2">
                        <input
                            class="border rounded p-1"
                            type="text"
                            prop:placeholder="Enter item"
                            prop:value=item
                            on:input=handle_input
                        />

                        <button class="border rounded p-1" on:click=handle_submit>
                            Submit
                        </button>
                    </div>
                </div>
            </main>
        </div>
    }
}

fn main() {
    leptos_dom::mount_to_body(App);
}
