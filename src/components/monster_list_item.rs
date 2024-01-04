use yew::prelude::*;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct MonsterListItemProps {
    #[prop_or(None)]
    pub name: Option<AttrValue>,
    #[prop_or(Vec::new())]
    pub family: Vec<AttrValue>,
    #[prop_or(None)]
    pub rank: Option<AttrValue>,
    #[prop_or(1)]
    pub count: usize,
}

#[function_component(MonsterListItem)]
pub fn monster_list_item(props: &MonsterListItemProps) -> Html {
    html! {
        <li class="list-group-item">
            <div class="row">
                <div class="col-8">
                {
                    match &props.name {
                        Some(name) => name.to_string(),
                        None => match props.family.len() > 0 {
                            true => props.family.join(" / "),
                            _ => "".to_string(),
                        },
                    }
                }
                </div>
                <div class="col-2">
                {
                    match &props.rank {
                        Some(rank) => html!{
                            <div class="badge text-bg-primary">
                                {format!("rank: {:}", rank)}
                            </div>
                        },
                        None => html!{},
                    }
                }
                </div>
                <div class="col-2">
                    {format!("x {:}", &props.count)}
                </div>
            </div>
        </li>
    }
}
