#![recursion_limit = "512"]
#[warn(unused_imports)]
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

//use stdweb::web::event::{
//    IEvent, IKeyboardEvent, KeyDownEvent, KeyPressEvent, KeyUpEvent, KeyboardLocation, MouseButton,
//    MouseDownEvent, MouseMoveEvent, MouseUpEvent,
//};

use stdweb::web::event::IKeyboardEvent;

type Context = ();

struct Model {
    input: String,
    edit_input: String,
    todos: Vec<ToDo>,
}

struct ToDo {
    text: String,
    edit: bool,
}

enum Msg {
    Add,
    Update(String),
    Remove(usize),
    Edit(usize),
    UpdateEdit(String),
    Toggle(usize),
    RemoveAll,
    Nothing,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            todos: vec![],
            input: String::from(""),
            edit_input: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                let to_do = ToDo {
                    text: self.input.clone(),
                    edit: false,
                };

                self.todos.push(to_do);
                self.input = String::from("");
                true
            }
            Msg::Update(string) => {
                self.input = string;
                true
            }
            Msg::Remove(index) => {
                self.todos.remove(index);
                true
            }
            Msg::RemoveAll => {
                self.todos = vec![];
                true
            }
            Msg::UpdateEdit(string) => {
                self.edit_input = string;
                true
            }
            Msg::Edit(index) => {
                let value = ToDo {
                    text: self.edit_input.clone(),
                    edit: true,
                };

                self.todos.remove(index);
                self.todos.push(value);
                true
            }
            Msg::Toggle(index) => {
                let todo = self.todos.get_mut(index).unwrap();
                todo.edit = !todo.edit;
                true
            }
            Msg::Nothing => false,
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let view_todo_edit = |(index, todo): (usize, &ToDo)| {
            if todo.edit == true {
                html! {
                <input class="edit"
                   type="text"
                   value=&todo.text
                   oninput=|e| Msg::UpdateEdit(e.value)
                   onblur=|_| Msg::Edit(index)
                   onkeypress=|e| {
                      if e.key() == "Enter" { Msg::Edit(index) } else { Msg::Nothing }
                   } />
                                }
            } else {
                html! {
                        <label ondoubleclick=move|_| Msg::Toggle(index), > {format!("{} ", &todo.text)}
                        </label>
                }
            }
        };

        let view_todo = |(index, todo): (usize, &ToDo)| {
            html! {
                <>
                    <li>
                        { view_todo_edit((index, &todo))}

                    <button onclick = move |_| Msg::Remove(index),>{"X"}</button>
                    </li>
                </>
            }
        };

        html! {
                                        <>
                                            <div>
                                                <h1>{"Todo App"}</h1>
                                                <input
                                                    placeholder="what do you want to do?",
                                                    value=&self.input,
                                                    oninput=|e| Msg::Update(e.value),
                                   onkeypress=|e| {
                                       if e.key() == "Enter" { Msg::Add } else { Msg::Nothing } />}
                                            </div>
                                            <div>
                                              <button onclick = |_| Msg::RemoveAll, >{"Delete all Todos!"}</button>
                                            </div>
                                            <div>
                                                <ul>
                                                {for self.todos.iter().enumerate().map(view_todo)}
                                                </ul>
                                            </div>
                                    </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
