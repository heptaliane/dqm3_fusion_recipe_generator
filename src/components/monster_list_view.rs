use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use yew::prelude::*;
use yew::Properties;

use super::super::data::Monster;
use super::super::recipe::MonsterNode;
use super::cards::Card;
use super::monster_list::MonsterList;

#[derive(Properties, PartialEq)]
pub struct MonsterListViewProps {
    pub monster: Option<Rc<RefCell<MonsterNode>>>,
    pub monster_lut: Rc<HashMap<usize, Monster>>,
}

#[function_component(MonsterListView)]
pub fn monster_list_view(props: &MonsterListViewProps) -> Html {
    html! {
        <Card>
        {
            match &props.monster {
                Some(monster) => html! {
                    <MonsterList
                        monster={monster.clone()}
                        monster_lut={props.monster_lut.clone()}
                    />
                },
                None => html! {},
            }
        }
        </Card>
    }
}
