use crate::switch::{AppAnchor, AppRoute};
use crate::types::Menu;
use log::debug;
use yew::prelude::*;

#[derive(Clone)]
pub struct MenuItem {
    link: ComponentLink<Self>,
    props: Props,
    //当前菜单是否处于选中
    menu_active: bool,
}

#[derive(Clone, Properties)]
pub struct Props {
    //菜单项
    pub menu_list: Menu,
    //回调-把当前点击的菜单id 回传到父组件，父组件更新所有子组件，子组件根据当前选择的菜单id,设置样式
    pub callback: Callback<u32>,
    //当前选择的菜单id
    pub select_menu_id: u32,
}

#[derive(Clone)]
pub enum Msg {
    ToggleMenu(u32),
    CurrentRoute(Option<AppRoute>),
}
impl Component for MenuItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            menu_active: false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.props = _props;
        //属性 变更，根据属性值，设置菜单选中状态
        if self.props.select_menu_id == self.props.menu_list.id {
            self.menu_active = !self.menu_active;
        } else {
            self.menu_active = false;
        }
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleMenu(id) => {
                self.props.callback.emit(id);
                true
            }
            Msg::CurrentRoute(route) => {
                debug!("{:#?}", route);
                false
            }
        }
    }

    fn view(&self) -> Html {
        let cls = MenuClass::get_class(self.menu_active);
        html! {
            <div>
                <div class=classes!("flex","items-center","px-6","py-2","duration-200","border-l-4","cursor-pointer", cls.menu_class)  onclick=self.clone().onclick()>
                    <svg class="w-5 h-5" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M2 10C2 5.58172 5.58172 2 10 2V10H18C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10Z" fill="currentColor"></path>
                        <path d="M12 2.25195C14.8113 2.97552 17.0245 5.18877 17.748 8.00004H12V2.25195Z" fill="currentColor"></path>
                    </svg>
                    <span class="mx-4">{&self.props.menu_list.name}</span>
                    <svg class=classes!("ml-auto","w-5","h-5","transition-transform","transform",cls.icon_class)  xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                    </svg>
                </div>
                //子菜单
                <div class=classes!("px-6","py-2","duration-200","bg-gray-900",cls.child_class)>
                    {
                        for self.props.menu_list.items.clone().iter().map(|m|{
                            html!{
                                <span class="block mt-2 px-7 py-1 text-gray-500 hover:bg-gray-600 hover:bg-opacity-25 hover:text-gray-100 cursor-pointer">

                                <AppAnchor route=AppRoute::PostList>
                                {m.name.clone()}
                                </AppAnchor>
                                </span>

                            }
                        })
                    }

                </div>
            </div>
        }
    }
}

struct MenuClass {
    //一级菜单 样式
    menu_class: String,
    //二级菜单 样式
    child_class: String,
    //箭头图片
    icon_class: String,
}

impl MenuClass {
    fn get_class(active: bool) -> Self {
        if active {
            Self {
                menu_class: "menu_active".to_string(),
                child_class: "".to_string(),
                icon_class: "-rotate-90".to_string(),
            }
        } else {
            Self {
                menu_class: "menu_normal".to_string(),
                child_class: "hidden".to_string(),
                icon_class: "".to_string(),
            }
        }
    }
}
impl MenuItem {
    //菜单点击事件
    fn onclick(self) -> Callback<MouseEvent> {
        let r = Msg::ToggleMenu(self.props.menu_list.id);
        self.link.callback(move |_ev: MouseEvent| r.clone())
    }
}
