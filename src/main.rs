use std::rc::Rc;
use std::cell::RefCell;

use wasm_logger;
use yew::prelude::*;

mod components;
mod data;
mod recipe;


#[function_component]
fn App() -> Html {
    let monster_lut = data::get_monster_data();
    let builder = recipe::MonsterTreeBuilder::new(monster_lut.clone());

    // let tree = builder.build(58);
    let tree = builder.build(522);

    html! {
        <components::monster_tree_view::MonsterTreeView
            monster_lut={Rc::new(monster_lut.clone())}
            monster={Rc::new(RefCell::new(tree))}
        />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
