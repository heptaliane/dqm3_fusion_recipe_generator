use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use yew::{Callback, Properties};

use super::super::data::{get_lang_data, get_rank_data};
use super::select::Select;

#[derive(Properties, PartialEq)]
pub struct MonsterScoutableSelectionProps {
    pub onchange: Callback<Option<usize>>,
}

#[function_component(MonsterScoutableSelection)]
pub fn monster_scoutable_selection(props: &MonsterScoutableSelectionProps) -> Html {
    let lang = get_lang_data();
    let ranks = get_rank_data();

    let enabled = use_state(|| false);
    let rank_value = use_state(|| 0);

    let handle_toggle = {
        let onchange = props.onchange.clone();
        let enabled = enabled.clone();
        let rank_value = rank_value.clone();

        Callback::from(move |e: Event| {
            let target = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            match target {
                Some(inp) if inp.checked() => {
                    enabled.set(true);
                    onchange.emit(Some(*rank_value));
                }
                _ => {
                    enabled.set(false);
                    onchange.emit(None);
                }
            }
        })
    };
    let handle_select = {
        let onchange = props.onchange.clone();
        let rank_value = rank_value.clone();
        Callback::from(move |k: String| {
            let rank: usize = k.parse().unwrap();
            rank_value.set(rank);
            onchange.emit(Some(rank));
        })
    };

    html! {
        <div>
            <div class="form-check form-switch">
                <input
                    class="form-check-input"
                    type="checkbox"
                    checked={*enabled}
                    onchange={handle_toggle}
                />
                <label class="form-check-label">
                    {lang["scoutable_title"].ja.clone()}
                </label>
            </div>
            <div>
            {
                match *enabled {
                    true => html!{
                        <Select
                            options={
                                let mut opts = ranks
                                    .iter()
                                    .map(|(k, v)| (k.to_string(), v.clone()))
                                    .collect::<Vec<(String, String)>>();
                                opts.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
                                opts
                            }
                            onchange={handle_select}
                        />
                    },
                    false => html!{},
                }
            }
            </div>
        </div>
    }
}
