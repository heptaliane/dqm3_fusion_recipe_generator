use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use yew::prelude::*;
use yew::Properties;

use super::super::recipe::MonsterNode;
use super::super::data::{get_lang_data, Monster};
use super::monster_tree::MonsterTree;
use super::cards::Card;

#[derive(Properties, PartialEq)]
pub struct MonsterTreeViewProps {
    pub monster: Rc<RefCell<MonsterNode>>,
    pub monster_lut: Rc<HashMap<usize, Monster>>,
}

#[function_component(MonsterTreeView)]
pub fn monster_tree_view(props: &MonsterTreeViewProps) -> Html {
    html! {
        <Card header={get_lang_data()["monster_tree"].ja.clone()}>
            <div class="monster-tree-container">
                <MonsterTree
                    monster={props.monster.clone()}
                    monster_lut={props.monster_lut.clone()}
                />
            </div>
        </Card>
    }
}
