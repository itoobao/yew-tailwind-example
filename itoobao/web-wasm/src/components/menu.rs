use log::debug;
use yew::prelude::*;

use crate::types::Menu as TypeMenu;

use super::MenuItem;
pub struct Menu {
    link: ComponentLink<Self>,
    //当前选中的菜单项id
    menu_id: u32,
    menu_click: Callback<u32>,
}

pub enum Msg {
    ToggleMenu(u32),
}
impl Component for Menu {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            menu_id: 0,
            menu_click: link.callback(Msg::ToggleMenu),
            link,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleMenu(id) => {
                self.menu_id = id;
            }
        }
        true
    }

    #[allow(unused)]
    fn view(&self) -> Html {
        let menu_list = TypeMenu::menu_list();
        html! {
            <div class="flex">
                <div class="hidden fixed inset-0 z-20 transition-opacity bg-black opacity-50 lg:hidden"></div>
                <div class="-translate-x-full ease-in fixed inset-y-0 left-0 z-30 w-64 overflow-y-auto transition duration-300 transform bg-gray-900 lg:translate-x-0 lg:static lg:inset-0">
                    <div class="flex items-center justify-center mt-8">
                        <svg class="w-12 h-12" viewBox="0 0 512 512" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path d="M364.61 390.213C304.625 450.196 207.37 450.196 147.386 390.213C117.394 360.22 102.398 320.911 102.398 281.6C102.398 242.291 117.394 202.981 147.386 172.989C147.386 230.4 153.6 281.6 230.4 307.2C230.4 256 256 102.4 294.4 76.7999C320 128 334.618 142.997 364.608 172.989C394.601 202.981 409.597 242.291 409.597 281.6C409.597 320.911 394.601 360.22 364.61 390.213Z" fill="#4C51BF" stroke="#4C51BF" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path>
                            <path d="M201.694 387.105C231.686 417.098 280.312 417.098 310.305 387.105C325.301 372.109 332.8 352.456 332.8 332.8C332.8 313.144 325.301 293.491 310.305 278.495C295.309 263.498 288 256 275.2 230.4C256 243.2 243.201 320 243.201 345.6C201.694 345.6 179.2 332.8 179.2 332.8C179.2 352.456 186.698 372.109 201.694 387.105Z" fill="white"></path>
                        </svg>
                        <span class="mx-2 text-2xl font-semibold text-white">{"Dashboard"}</span>
                    </div>
                    <nav class="mt-10">
                        {
                            for menu_list.iter().map(|m|{
                                html!{
                                    <MenuItem menu_list=m.clone() callback=&self.menu_click select_menu_id=self.menu_id />
                                }
                            })
                        }

                    </nav>
                </div>

            </div>
        }
    }
}
