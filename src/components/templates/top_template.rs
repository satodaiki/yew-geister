use crate::{
    components::atoms::icon::{Icon, IconType, IconSize, IconColor},
    components::organisms::list::{List},
};

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::{prelude::*, Switch};

pub enum Msg {
    OnClick,
}

pub struct TopTemplate {
    link: ComponentLink<Self>,
}
impl Component for TopTemplate {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
    
        html! {
            <div>
                <div class="title">
                    <div class="title__icon">
                        <div class="title__icon__container">
                            <Icon
                                name=IconType::Ghost
                                color=IconColor::Red
                            />
                            <Icon
                                name=IconType::Ghost
                                color=IconColor::Blue
                            />
                        </div>
                    </div>
                    <div class="title__name">{ "Geister" }</div>
                </div>
                <div class="game-info">
                    <div class="game-info__player">
                        <div class="label-text-field">
                            <label for="player_name" class="game-info__player__name">{ "プレイヤー名" }</label>
                            <input type="text" id="player_name" class="game-info__player__text-field text-field" />
                        </div>
                    </div>
                    <div class="game-info__battle-select">
                        <List
                        />
                    </div>
                    <div class="game-info__twitter">
                        <Icon
                            name=IconType::Twitter
                            size=IconSize::Small
                        />
                    </div>
                </div>
            </div>
        }
    }
}