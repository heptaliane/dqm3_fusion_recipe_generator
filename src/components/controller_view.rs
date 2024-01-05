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
    let handle_id_change = {
        let onchange = props.onchange.clone();
        let condition = props.condition.clone();
        Callback::from(move |id: Option<usize>| {
            onchange.emit(condition.with_monster_id(id));
        })
    };
    let handle_scoutable_rank_change = {
        let onchange = props.onchange.clone();
        let condition = props.condition.clone();
        Callback::from(move |rank: Option<usize>| {
            onchange.emit(condition.with_scoutable_limit_rank(rank));
        })
    };

    html! {
        <Card header={lang["controller_header"].ja.clone()}>
            <MonsterInput
                monster_lut={props.monster_lut.clone()}
                onchange={handle_id_change}
            />
            <MonsterScoutableSelection
                onchange={handle_scoutable_rank_change}
            />
        </Card>
    }
}
