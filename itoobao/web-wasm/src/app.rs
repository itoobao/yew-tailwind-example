use crate::components::{Header, SideBar};
use crate::{pages::Index, routes::AppRoute, services::is_authenticated};
use yew::prelude::*;
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::{Route, RouteAgent, RouteService},
    Switch,
};
pub struct App {
    current_route: Option<AppRoute>,
    route_agent: Box<dyn Bridge<RouteAgent>>,
}

pub enum Msg {
    Route(Route),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let route_service: RouteService<Route> = RouteService::new();
        let route = route_service.get_route();

        let route_agent = RouteAgent::bridge(_link.callback(Msg::Route));
        Self {
            current_route: AppRoute::switch(route),
            route_agent,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn rendered(&mut self, _first_render: bool) {
        if _first_render && !is_authenticated() {
            self.route_agent.send(ChangeRoute(AppRoute::Login.into()));
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        //检测是否登录
        // if is_authenticated() {
        //     //路由到子模块
        //     if let Some(route) = &self.current_route {
        //         match route {
        //             AppRoute::Login => html! {"logining"},
        //             AppRoute::Home => html! {<Index />},
        //         }
        //     } else {
        //         //404
        //         html! {"No child component available"}
        //     }
        // } else {
        //     html! {<Login />}
        // }
        html! {
            <div class="flex h-screen bg-gray-300 font-sans">
                <SideBar />
                <div class="flex-1 flex flex-col overflow-hidden">
                    <Header />
                </div>
            </div>
        }
    }
}
