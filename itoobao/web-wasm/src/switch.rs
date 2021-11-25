use yew::{html::IntoPropValue, web_sys::Url};
use yew_router::{prelude::*, switch::Permissive};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/login"]
    Login,

    #[to = "/posts/{}"]
    Post(u64),

    #[to = "/posts/?page={}"]
    PostListPage(u64),

    #[to = "/posts/"]
    PostList,

    #[to = "/authors/{id}"]
    Author(u64),

    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),

    #[to = "/!"]
    Home,
}
impl AppRoute {
    pub fn into_public(self) -> PublicUrlSwitch {
        PublicUrlSwitch(self)
    }

    pub fn into_route(self) -> Route {
        Route::from(self.into_public())
    }
}

/// 工具类，包裹AppRoute,处理公共url前缀信息。
/// 例如: 公共url前缀为 "/examples/",而不是 "/"时
#[derive(Clone, Debug)]
pub struct PublicUrlSwitch(AppRoute);

impl PublicUrlSwitch {
    fn base_url() -> Url {
        if let Ok(Some(href)) = yew::utils::document().base_uri() {
            //把绝对url,转换为Url对象。
            Url::new(&href).unwrap()
        } else {
            Url::new("/").unwrap()
        }
    }

    fn base_path() -> String {
        let mut path = Self::base_url().pathname();
        if path.ends_with("/") {
            //路径 若以“/”结果，则去掉"/", AppRoute已经处理了
            path.pop();
        }
        path
    }

    pub fn route(self) -> AppRoute {
        self.0
    }
}

impl Switch for PublicUrlSwitch {
    fn from_route_part<STATE>(part: String, state: Option<STATE>) -> (Option<Self>, Option<STATE>) {
        let (route, state) = AppRoute::from_route_part(part.to_owned(), state);
        (route.map(Self), state)
        // if let Some(part) = part.strip_prefix(&Self::base_path()) {
        //     let (route, state) = AppRoute::from_route_part(part.to_owned(), state);
        //     (route.map(Self), state)
        // } else {
        //     (None, None)
        // }
    }

    fn build_route_section<STATE>(self, route: &mut String) -> Option<STATE> {
        //route.push_str(&Self::base_path());
        self.0.build_route_section(route)
    }
}

impl IntoPropValue<PublicUrlSwitch> for AppRoute {
    fn into_prop_value(self) -> PublicUrlSwitch {
        self.into_public()
    }
}

pub type AppRouter = Router<PublicUrlSwitch>;
pub type AppAnchor = RouterAnchor<PublicUrlSwitch>;
