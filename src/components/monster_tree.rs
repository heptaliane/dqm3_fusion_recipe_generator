use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;
use yew::Properties;

use super::super::data::Monster;
use super::super::recipe::MonsterNode;
use super::monster_tree_node::MonsterTreeNode;

#[derive(Properties, PartialEq)]
pub struct MonsterTreeProps {
    pub monster_lut: Rc<HashMap<usize, Monster>>,
    pub monster: Rc<RefCell<MonsterNode>>,
}

#[function_component(MonsterTree)]
pub fn monster_tree(props: &MonsterTreeProps) -> Html {
    let monster = match props.monster.borrow().data.monster_id {
        Some(id) => match props.monster_lut.get(&id) {
            Some(m) => Some(m.name.clone()),
            None => None,
        },
        None => None,
    };
    let family = match props.monster.borrow().data.spec.clone() {
        Some(spec) => Some(spec.family),
        None => None,
    };
    let children = props.monster.borrow().children.clone();

    html! {
        <div>
            <MonsterTreeNode
                monster={monster}
                family={family}
            />
            <div class="monster-nodes-container">
            {
                children.iter().map(|m| {
                    html! {
                        <div class="monster-node-container">
                            <MonsterTree
                                monster_lut={props.monster_lut.clone()}
                                monster={m.clone()}
                            />
                        </div>
                    }
                }).collect::<Html>()
            }
            </div>
        </div>
    }
}
