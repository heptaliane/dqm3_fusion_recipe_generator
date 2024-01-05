use std::collections::HashMap;
use std::rc::Rc;

use yew::prelude::*;
use yew::{Callback, Properties};

use super::super::data::{get_lang_data, Monster};
use super::cards::Card;
use super::monster_input::MonsterInput;

#[derive(Clone, PartialEq)]
pub struct SearchConditions {
    pub monster_id: Option<usize>,
}

#[derive(Properties, PartialEq)]
pub struct ControllerViewProps {
    pub condition: SearchConditions,
    pub monster_lut: Rc<HashMap<usize, Monster>>,

    pub onchange: Callback<SearchConditions>,
}

#[function_component(ControllerView)]
pub fn controller_view(props: &ControllerViewProps) -> Html {
    let lang = get_lang_data();
    let handle_id_change = props.onchange.clone();

    html! {
        <Card header={lang["controller_header"].ja.clone()}>
            <MonsterInput
                monster_lut={props.monster_lut.clone()}
                onchange={
                    Callback::from(move |id: Option<usize>| {
                        handle_id_change.emit(SearchConditions {
                            monster_id: id,
                        });
                    })
                }
            />
        </Card>
    }
}
