use yew_router::{route::Route, Switch};

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/imihu/"]
    Imihu,
    #[to = "/test1/"]
    Test1,
    #[to = "/"]
    Index,
}