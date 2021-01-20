use yew_router::{route::Route, Switch};

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/test"]
    Test,
    #[to = "/"]
    Index,
}