#![recursion_limit = "256"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::{prelude::*, Switch};
// use mdc_yew::prelude::*;

mod components;
use components::atoms::button::Button;
mod switch;
use switch::AppRoute;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        // type Anchor = RouterAnchor<AppRoute>;
    
        html! {
            // <div>
            <div>
                <RouterButton<AppRoute> route=AppRoute::Test1> {"test_one"} </RouterButton<AppRoute>>
                <RouterButton<AppRoute> route=AppRoute::Imihu> {"test_two"} </RouterButton<AppRoute>>
                <RouterButton<AppRoute> route=AppRoute::Index> {"/"} </RouterButton<AppRoute>>
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::Imihu => html! {
                                <Button
                                    label="testlabel2"
                                    value="aaa"
                                    // onclick=self.link.callback(|_| Msg::AddOne)
                                />
                            },
                            AppRoute::Test1 => html! {
                                <Button
                                    label="testlabel1"
                                    value="aaa"
                                    // onclick=self.link.callback(|_| Msg::AddOne)
                                />
                            },
                            AppRoute::Index => html! {
                                <div>{ "test router" }</div>
                            },
                        }
                    })
                />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}