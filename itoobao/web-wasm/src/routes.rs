use yew_router::prelude::*;

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/login"]
    Login,

    #[to = "/"]
    Home,
}
