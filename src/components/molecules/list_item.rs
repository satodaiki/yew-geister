use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::Properties;
use yew::web_sys;

use crate::components::atoms::icon::{Icon, IconType, IconSize, IconColor};

pub enum Msg {
    OnMouseover,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub text: &'static str,
    pub onmouseover: Callback<()>
}

pub struct ListItem {
    props: Props,
    link: ComponentLink<Self>,
}
impl Component for ListItem {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnMouseover => {
                self.props.onmouseover.emit(());
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
            <div class="list_item" onmouseover=self.link.callback(|_| Msg::OnMouseover)>
                <div>{ self.props.text }</div>
            </div>
        }
    }
}