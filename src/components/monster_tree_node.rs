use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew::Properties;

use super::super::data::get_family_data;
use super::cards::Card;

#[derive(Properties, PartialEq)]
pub struct MonsterTreeNodeProps {
    #[prop_or(None)]
    pub monster: Option<AttrValue>,

    #[prop_or(None)]
    pub family: Option<usize>,
}

#[function_component(MonsterTreeNode)]
pub fn monster_tree_node(props: &MonsterTreeNodeProps) -> Html {
    html! {
        <Card variant="color-dark monster-node">
            {
                match props.monster.clone() {
                    Some(name) => html!{name},
                    _ => html!{},
                }
            }
            {
                match props.family.clone() {
                    Some(family) => match get_family_data().get(&family) {
                        Some(name) => html!{format!("{:} 系", name)},
                        _ => html!{},
                    },
                    _ => html!{},
                }
            }
        </Card>
    }
}
