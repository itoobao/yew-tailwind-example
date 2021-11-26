use crate::components::{BreadCrumbs, Header, Menu};
use crate::switch::{AppRoute, AppRouter, PublicUrlSwitch};
use crate::{pages::Login, services::is_authenticated};
use yew::prelude::*;
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::{Route, RouteAgent},
    switch::Permissive,
};
pub struct App {
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

pub enum Msg {
    Route(Route),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(_link.callback(Msg::Route));
        Self { router_agent }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn rendered(&mut self, _first_render: bool) {
        if _first_render && !is_authenticated() {
            //第一次渲染，未登录 跳转到登录页
            self.router_agent
                .send(ChangeRoute(AppRoute::Login.into_route()));
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Route(route) => {}
        }
        true
    }

    fn view(&self) -> Html {
        if is_authenticated() {
            html! {
                <div class="flex h-screen bg-gray-300 font-sans">
                    <Menu />
                    <div class="flex-1 flex flex-col overflow-hidden">
                        <Header />

                        <main class="flex-1 overflow-hidden overflow-y-auto bg-gray-200">
                            <div class="container mx-auto bg-white h-full">
                            <BreadCrumbs />
                            <AppRouter render=AppRouter::render(Self::switch) redirect=AppRouter::redirect(|route: Route| {
                                AppRoute::PageNotFound(Permissive(Some(route.route))).into_public()
                            })
                            />
                            </div>

                        </main>
                    </div>

                </div>

            }
        } else {
            html! {
                <Login />
            }
        }
    }
}

impl App {
    fn switch(switch: PublicUrlSwitch) -> Html {
        match switch.route() {
            AppRoute::Login => {
                html! {<Login />}
            }
            AppRoute::Post(id) => {
                html! {}
            }
            AppRoute::PostListPage(page) => {
                html! {}
            }
            AppRoute::PostList => {
                html! {"list"}
            }
            AppRoute::Author(id) => {
                html! {}
            }
            AppRoute::Home => {
                html! {"home"}
            }
            AppRoute::PageNotFound(Permissive(route)) => {
                html! {
                    "not found"
                }
            }
        }
    }
}
