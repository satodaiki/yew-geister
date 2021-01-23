#![recursion_limit = "256"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::{prelude::*, Switch};

pub struct TopTemplate {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for TopTemplate {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
    
        html! {
            <div>
                <input type="text" class="text-field" />
            </div>
        }
    }
}