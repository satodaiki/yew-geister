use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::Properties;
use yew::web_sys;

use crate::{
    components::atoms::icon::{Icon, IconType, IconSize, IconColor},
    components::molecules::list_item::{ListItem},
};

pub enum Msg {
    OnMouseover(i32),
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {

    // pub text: &'static str,
    // pub onclick: Callback<()>
}

pub struct List {
    props: Props,
    link: ComponentLink<Self>,
    lists: Vec<&'static str>,
    select_element: i32,
}
impl Component for List {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            lists: vec!["ローカル対戦", "CPU対戦"],
            select_element: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnMouseover(value) => {
                self.select_element = value;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="list">
                { self.list_container() }
            </div>
        }
    }
}
impl List {
    fn list_container(&self) -> Html {
        let list_container = self.lists.iter().enumerate().map(|(i, text)| {
            let index: i32 = i as i32;
            html! {
                <div class="list__container">
                    <div class="list__container__icon">
                    {
                        if self.select_element == index {
                            html! {
                                <Icon
                                    name=IconType::Ghost
                                    size=IconSize::Small
                                />
                            }
                        } else {
                            html! {}
                        }
                    }
                    </div>
                    <div class="list__container__item">
                        <ListItem
                            text=text
                            onmouseover=self.link.callback(move |_| Msg::OnMouseover(index))
                        />
                    </div>
                </div>
            }
        });
        html! { { for list_container } }
    }
}