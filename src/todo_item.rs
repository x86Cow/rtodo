use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Todo {
    pub id: usize,
    pub title:String,
    pub desc: String,
    pub status: bool,
}


#[derive(Properties, PartialEq)]
pub struct TodosListProps {
    pub todos: Vec<Todo>,
}


#[function_component(TodosList)]
pub fn todos_list(TodosListProps { todos }: &TodosListProps) -> Html {

    todos.iter().map(|todo| html! {
        <p key={todo.id}>{format!("{}: {}", todo.title, todo.desc)}</p>
    }).collect()
}

