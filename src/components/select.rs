use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlSelectElement};
use yew::prelude::*;
use yew::{Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct SelectProps {
    pub options: Vec<(String, String)>,
    pub onchange: Callback<String>,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let handle_select = {
        let onchange = props.onchange.clone();
        Callback::from(move |e: Event| {
            let target = e.target().and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(select) = target {
                onchange.emit(select.value());
            }
        })
    };

    html!{
        <select
            class="form-select"
            onchange={handle_select}
        >
            {
                props.options.iter().map(|(k, v)| html! {
                    <option value={k.clone()}>{v}</option>
                }).collect::<Html>()
            }
        </select>
    }
}
