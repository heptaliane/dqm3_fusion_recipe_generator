use yew::prelude::*;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct MonsterTreeArrowProps {
    #[prop_or(1)]
    pub left: usize,
    #[prop_or(1)]
    pub right: usize,
}

#[function_component(MonsterTreeArrow)]
pub fn monster_tree_arrow(props: &MonsterTreeArrowProps) -> Html {
    let is_partial_single_arrow = (props.left == 1) || (props.right == 1);

    html! {
        <div
            class="grid-container"
            style={match is_partial_single_arrow {
                true => "grid-template-columns: repeat(2, 1fr);",
                false => "grid-template-columns: repeat(3, 1fr);",
            }}
        >
            <div class="arrow-inner-container">
                {html!{<div />}}
                {
                    (1..props.left).map(|_| {
                        html! {
                            <div class="arrow-border-right arrow-border-top" />
                        }
                    }).collect::<Html>()
                }
                {html!{<div class="arrow-border-top" />}}
            </div>
            {
                match is_partial_single_arrow {
                    true => html! {},
                    false => html! {
                        <div class="arrow-inner-container">
                            <div />
                            <div class="arrow-border-top" />
                        </div>
                    },
                }
            }
            <div class="arrow-inner-container">
                {html!{<div />}}
                {
                    (1..props.right).map(|_| {
                        html! {
                            <div class="arrow-border-left arrow-border-top" />
                        }
                    }).collect::<Html>()
                }
                {html!{<div class="arrow-border-top" />}}
            </div>
        </div>
    }
}
