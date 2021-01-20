use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::Properties;
use yew::web_sys;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub label: &'static str,
    pub value: &'static str,
    // pub onclick: Callback<MouseEvent>,
}

pub struct Button {
    props: Props,
    link: ComponentLink<Self>,
}
impl Component for Button {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let Props {
            label,
            value,
            // ref onclick
        } = self.props;

        html! {
            <>
                <label>{ label }</label>
                <button /* onclick=onclick */>{ value }</button>
            </>
        }
    }
}