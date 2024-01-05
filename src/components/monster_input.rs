use std::collections::HashMap;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use yew::{Callback, Properties};

use super::super::data::{get_lang_data, Monster};

#[derive(Properties, PartialEq)]
pub struct MonsterInputView {
    pub monster_lut: Rc<HashMap<usize, Monster>>,

    pub onchange: Callback<Option<usize>>,
}

#[function_component(MonsterInput)]
pub fn monster_input(props: &MonsterInputView) -> Html {
    let lang = get_lang_data();
    let monster_ids: HashMap<String, usize> = props
        .monster_lut
        .iter()
        .map(|(&k, v)| (v.name.clone(), k))
        .collect();
    let handle_id_change = props.onchange.clone();
    html! {
        <div class="form-floating">
            <input
                type="text"
                id="monster_name_input"
                class="form-control"
                onchange={
                    Callback::from(move |e: Event| {
                        let target = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                        match target {
                            Some(inp) => handle_id_change.emit(monster_ids.get(&inp.value()).copied()),
                            _ => handle_id_change.emit(None),
                        }
                    })
                }
            />
            <label for="monster_name_input">
                {lang["monster_name_input"].ja.clone()}
            </label>
        </div>
    }
}
