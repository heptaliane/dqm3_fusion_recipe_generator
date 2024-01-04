use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use yew::prelude::*;
use yew::Properties;

use super::super::data::{get_family_data, get_rank_data, Monster};
use super::super::recipe::MonsterNode;
use super::monster_list_item::MonsterListItem;

#[derive(Properties, PartialEq)]
pub struct MonsterListProps {
    pub monster: Rc<RefCell<MonsterNode>>,
    pub monster_lut: Rc<HashMap<usize, Monster>>,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct LeafNodeData {
    monster_id: Option<usize>,
    family: Vec<usize>,
    rank: Option<usize>,
}

fn try_get_leaf_node_data(
    monster_lut: &HashMap<usize, Monster>,
    node: Rc<RefCell<MonsterNode>>,
) -> Option<LeafNodeData> {
    let data = &node.borrow().data;

    match data.monster_id {
        Some(id) => {
            let children = &node.borrow().children;
            let mut standard_children = children
                .iter()
                .filter_map(|c| c.borrow().data.spec.clone())
                .map(|s| s.family)
                .collect::<Vec<usize>>();
            standard_children.sort();
            let monster = &monster_lut[&id];

            match (standard_children.len(), children.len()) {
                (_, 0) => Some(LeafNodeData {
                    monster_id: Some(id),
                    family: vec![],
                    rank: None,
                }),
                (sc, ac) if sc == ac => Some(LeafNodeData {
                    monster_id: None,
                    family: standard_children,
                    rank: Some(monster.rank),
                }),
                _ => None,
            }
        }
        None => Some(LeafNodeData {
            monster_id: None,
            rank: None,
            family: match &data.spec {
                Some(spec) => vec![spec.family],
                _ => vec![],
            },
        }),
    }
}

fn get_leaf_node_data(
    monster_lut: &HashMap<usize, Monster>,
    root: Rc<RefCell<MonsterNode>>,
) -> HashMap<LeafNodeData, usize> {
    let mut leaves: HashMap<LeafNodeData, usize> = HashMap::new();
    let mut stack = vec![root];

    while stack.len() > 0 {
        let cursor = std::mem::replace(&mut stack, Vec::new());
        for node in cursor {
            match try_get_leaf_node_data(monster_lut, node.clone()) {
                Some(leaf) => {
                    match leaves.get(&leaf) {
                        Some(cnt) => leaves.insert(leaf, cnt + 1),
                        None => leaves.insert(leaf, 1),
                    };
                }
                None => {
                    stack.extend(node.borrow().children.clone());
                }
            }
        }
    }

    leaves
}

#[function_component(MonsterList)]
pub fn monster_list(props: &MonsterListProps) -> Html {
    let leaves = get_leaf_node_data(props.monster_lut.as_ref(), props.monster.clone());
    let families = get_family_data();
    let ranks = get_rank_data();

    html! {
        <ul class="list-group">
        {
            leaves.iter().map(|(leaf, cnt)| html! {
                <MonsterListItem
                    name={match leaf.monster_id {
                        Some(id) => Some(AttrValue::from(props.monster_lut[&id].name.clone())),
                        None => None,
                    }}
                    family={
                        leaf.family.iter().map(|id|
                            AttrValue::from(families[&id].clone())
                        ).collect::<Vec<AttrValue>>()
                    }
                    rank={match leaf.rank {
                        Some(id) => Some(AttrValue::from(ranks[&id].clone())),
                        None => None,
                    }}
                    count={cnt}
                />
            }).collect::<Html>()
        }
        </ul>
    }
}
