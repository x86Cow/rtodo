use yew::prelude::*;

mod todo_item;
use todo_item::*;

#[function_component(App)] fn app() -> Html {
    // Create TODOS
    let todos = vec![
        Todo {
            id: 1,
            title: "Buy a house".to_string(),
            desc: "we need a new house!".to_string(),
            status: true,
        },
        Todo {
            id: 2,
            title: "Buy a shed".to_string(),
            desc: "we need a new shed!".to_string(),
            status: false,
        },
        Todo {
            id: 3,
            title: "Fold Laundry".to_string(),
            desc: "Fold laundry before packing".to_string(),
            status: false,
        }
    ];



    // load to HTML
    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
            <h3>{"Todos"}</h3>
            <TodosList todos = { todos } />
            </div>
            </>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
