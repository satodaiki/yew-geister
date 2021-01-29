use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::Properties;
use yew::web_sys;

#[derive(Clone, Debug, PartialEq)]
pub enum IconType {
    Ghost,
    Twitter,
}

#[derive(Clone, Debug, PartialEq)]
pub enum IconSize {
    Large,
    Small,
}

#[derive(Clone, Debug, PartialEq)]
pub enum IconColor {
    Red,
    Blue,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub name: IconType,
    #[prop_or_default]
    pub size: Option<IconSize>,
    #[prop_or_default]
    pub color: Option<IconColor>,
}

pub struct Icon {
    props: Props,
    link: ComponentLink<Self>,
    classes: Vec<&'static str>,
    img_classes: Vec<&'static str>,
}
impl Component for Icon {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut classes = vec!["icon"];
        let mut img_classes = vec!["icon__img"];

        if let Some(v) = &props.size {
            match v {
                IconSize::Large => classes.push("icon--large"),
                IconSize::Small => classes.push("icon--small"),
            }
        }

        if let Some(v) = &props.color {
            match v {
                IconColor::Red => img_classes.push("icon__img--red"),
                IconColor::Blue => img_classes.push("icon__img--blue"),
            }
        }

        Self {
            props,
            link,
            classes,
            img_classes,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class=self.classes.clone()>
                    {
                        match self.props.name {
                            IconType::Ghost => html! {
                                <svg class=self.img_classes.clone() xmlns="http://www.w3.org/2000/svg" width="56.25" height="75" viewBox="0 0 56.25 75">
                                    <path id="Icon_awesome-ghost" data-name="Icon awesome-ghost" d="M27.261.013C11.867.475,0,13.9,0,29.3v38.66a2.343,2.343,0,0,0,4,1.657l3.65-2.714a2.345,2.345,0,0,1,3.151.324l6.292,7.083a2.344,2.344,0,0,0,3.315,0L26.373,67.6a2.342,2.342,0,0,1,3.5,0l5.965,6.716a2.344,2.344,0,0,0,3.315,0l6.292-7.083a2.343,2.343,0,0,1,3.151-.324l3.65,2.714a2.343,2.343,0,0,0,4-1.657V28.125A28.125,28.125,0,0,0,27.261.013Zm-8.511,32.8a4.688,4.688,0,1,1,4.688-4.688A4.688,4.688,0,0,1,18.75,32.813Zm18.75,0a4.688,4.688,0,1,1,4.688-4.688A4.688,4.688,0,0,1,37.5,32.813Z" transform="translate(0 0)" fill="#707070"/>
                                </svg>
                            },
                            IconType::Twitter => html! {
                                <svg class=self.img_classes.clone() xmlns="http://www.w3.org/2000/svg" width="43.094" height="35" viewBox="0 0 43.094 35">
                                    <path id="Icon_awesome-twitter" data-name="Icon awesome-twitter" d="M38.664,12.1c.027.383.027.766.027,1.148,0,11.676-8.887,25.129-25.129,25.129A24.959,24.959,0,0,1,0,34.416a18.271,18.271,0,0,0,2.133.109A17.688,17.688,0,0,0,13.1,30.752,8.848,8.848,0,0,1,4.84,24.627a11.138,11.138,0,0,0,1.668.137,9.341,9.341,0,0,0,2.324-.3A8.833,8.833,0,0,1,1.75,15.795v-.109a8.9,8.9,0,0,0,3.992,1.121A8.845,8.845,0,0,1,3.008,4.994a25.1,25.1,0,0,0,18.211,9.242A9.97,9.97,0,0,1,21,12.213,8.84,8.84,0,0,1,36.285,6.17a17.389,17.389,0,0,0,5.605-2.133A8.808,8.808,0,0,1,38.008,8.9a17.706,17.706,0,0,0,5.086-1.367,18.985,18.985,0,0,1-4.43,4.566Z" transform="translate(0 -3.381)" fill="#707070"/>
                                </svg>
                            },
                        }
                    }
                </div>
            </>
        }
    }
}