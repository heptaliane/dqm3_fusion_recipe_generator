use std::collections::HashMap;
use std::rc::Rc;

use yew::prelude::*;
use yew::{Callback, Properties};

use super::super::data::{get_lang_data, Monster};
use super::super::recipe::SearchCondition;
use super::cards::Card;
use super::monster_input::MonsterInput;
use super::monster_scoutable_selection::MonsterScoutableSelection;

#[derive(Properties, PartialEq)]
pub struct ControllerViewProps {
    pub condition: SearchCondition,
    pub monster_lut: Rc<HashMap<usize, Monster>>,

    pub onchange: Callback<SearchCondition>,
}

#[function_component(ControllerView)]
pub fn controller_view(props: &ControllerViewProps) -> Html {
    let lang = get_lang_data();
    let id_condition = props.condition.clone();
    let handle_id_change = props.onchange.clone();

    html! {
        <Card header={lang["controller_header"].ja.clone()}>
            <MonsterInput
                monster_lut={props.monster_lut.clone()}
                onchange={
                    Callback::from(move |id: Option<usize>| {
                        handle_id_change.emit(id_condition.with_monster_id(id));
                    })
                }
            />
            <MonsterScoutableSelection
                onchange={
                    Callback::from(move |rank: Option<usize>| {
                        log::info!("{:?}", rank);
                    })
                }
            />
        </Card>
    }
}
