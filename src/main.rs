use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use wasm_logger;
use yew::prelude::*;
use yew::Properties;

mod components;
mod data;
mod recipe;

#[derive(Properties, PartialEq, Default)]
pub struct AppProps;

pub enum AppMessage {
    ChangeSearchCondition(components::controller_view::SearchConditions),
}

pub struct App {
    search_condition: components::controller_view::SearchConditions,

    tree_builder: recipe::MonsterTreeBuilder,
    monster_lut: Rc<HashMap<usize, data::Monster>>,
}

impl Component for App {
    type Message = AppMessage;
    type Properties = AppProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let monster_lut = data::get_monster_data();

        App {
            search_condition: components::controller_view::SearchConditions { monster_id: None },
            tree_builder: recipe::MonsterTreeBuilder::new(monster_lut.clone()),
            monster_lut: Rc::new(monster_lut),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::ChangeSearchCondition(cond) => {
                self.search_condition = cond;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let tree = match self.search_condition.monster_id {
            Some(id) => Some(Rc::new(RefCell::new(self.tree_builder.build(id)))),
            None => None,
        };

        html! {
            <div class="container">
                <components::controller_view::ControllerView
                    condition={self.search_condition.clone()}
                    monster_lut={self.monster_lut.clone()}
                    onchange={
                        ctx.link().callback(|cond| Self::Message::ChangeSearchCondition(cond))
                    }
                />
                <components::monster_tree_view::MonsterTreeView
                    monster_lut={self.monster_lut.clone()}
                    monster={tree.clone()}
                />
                <components::monster_list_view::MonsterListView
                    monster_lut={self.monster_lut.clone()}
                    monster={tree.clone()}
                />
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
