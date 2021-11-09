use log::debug;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
pub struct SideBar {
    link: ComponentLink<Self>,
}

impl Component for SideBar {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
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
                        <div class="border-b">
                            <div class="flex items-center px-6 py-2 duration-200 border-l-4 menu_active cursor-pointer" onclick = self.link.callback(Self::menu_click)>
                                <svg class="w-5 h-5" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M2 10C2 5.58172 5.58172 2 10 2V10H18C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10Z" fill="currentColor"></path>
                                    <path d="M12 2.25195C14.8113 2.97552 17.0245 5.18877 17.748 8.00004H12V2.25195Z" fill="currentColor"></path>
                                </svg>
                                <span class="mx-4">{"Dashboard"}</span>
                            </div>
                            //子菜单
                            <div class="px-6 py-2 duration-200 bg-gray-900 hidden">
                                <span class="block mt-2 px-7 py-1 text-gray-500 hover:bg-gray-600 hover:bg-opacity-25 hover:text-gray-100 cursor-pointer">{"Dashboard"}</span>
                                <span class="block mt-2 px-7 py-1 text-gray-500 hover:bg-gray-600 hover:bg-opacity-25 hover:text-gray-100 cursor-pointer">{"Dashboard"}</span>
                                <span class="block mt-2 px-7 py-1 text-gray-500 hover:bg-gray-600 hover:bg-opacity-25 hover:text-gray-100 cursor-pointer">{"Dashboard"}</span>
                            </div>
                        </div>

                        <div class="border-b">
                            <div class="flex items-center px-6 py-2 duration-200 border-l-4 menu_normal cursor-pointer" onclick = self.link.callback(Self::menu_click)>
                                <svg class="w-5 h-5" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M5 3C3.89543 3 3 3.89543 3 5V7C3 8.10457 3.89543 9 5 9H7C8.10457 9 9 8.10457 9 7V5C9 3.89543 8.10457 3 7 3H5Z" fill="currentColor"></path>
                                    <path d="M5 11C3.89543 11 3 11.8954 3 13V15C3 16.1046 3.89543 17 5 17H7C8.10457 17 9 16.1046 9 15V13C9 11.8954 8.10457 11 7 11H5Z" fill="currentColor"></path>
                                    <path d="M11 5C11 3.89543 11.8954 3 13 3H15C16.1046 3 17 3.89543 17 5V7C17 8.10457 16.1046 9 15 9H13C11.8954 9 11 8.10457 11 7V5Z" fill="currentColor"></path>
                                    <path d="M11 13C11 11.8954 11.8954 11 13 11H15C16.1046 11 17 11.8954 17 13V15C17 16.1046 16.1046 17 15 17H13C11.8954 17 11 16.1046 11 15V13Z" fill="currentColor"></path>
                                </svg>

                                <span class="mx-4">{"UI Elements"}</span>
                                <svg class="ml-auto w-5 h-5 transition-transform transform"  xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                                </svg>
                            </div>
                            //子菜单
                            <div class="px-6 py-2 duration-200 bg-gray-900 hidden">
                                <span class="block mt-2 px-7 py-1 text-gray-500 hover:bg-gray-600 hover:bg-opacity-25 hover:text-gray-100 cursor-pointer">{"Dashboard"}</span>
                                <span class="block mt-2 px-7 py-1 text-gray-500 hover:bg-gray-600 hover:bg-opacity-25 hover:text-gray-100 cursor-pointer">{"Dashboard"}</span>
                                <span class="block mt-2 px-7 py-1 text-gray-500 hover:bg-gray-600 hover:bg-opacity-25 hover:text-gray-100 cursor-pointer">{"Dashboard"}</span>
                            </div>
                        </div>

                    </nav>
                </div>

            </div>
        }
    }
}

impl SideBar {
    #[allow(unused)]
    fn menu_click(e: MouseEvent) {
        //当前元素
        let element = e
            .current_target()
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();

        debug!("{}", element.class_name());
        //隐藏所有 非当前菜单的子菜单
        //当前元素的父节点的父节点的第一个子节点
        let mut node = element
            .parent_element()
            .unwrap()
            .parent_element()
            .unwrap()
            .first_element_child();
        //当前元素的下一个兄弟节点
        let sibling = element.next_element_sibling().unwrap();
        while node.is_some() {
            let n = &node.unwrap();
            n.children()
                .get_with_index(0)
                .unwrap()
                .class_list()
                .remove_1("menu_active");
            n.children()
                .get_with_index(0)
                .unwrap()
                .class_list()
                .add_1("menu_normal");
            //子菜单
            if let Some(child) = n.children().get_with_index(1) {
                //检测和当前元素的子菜单是否一样
                if !child.is_same_node(Some(&sibling)) {
                    child.class_list().add_1("hidden");
                } else {
                    n.children()
                        .get_with_index(0)
                        .unwrap()
                        .class_list()
                        .add_1("menu_active");
                    n.children()
                        .get_with_index(0)
                        .unwrap()
                        .children()
                        .get_with_index(2)
                        .unwrap()
                        .class_list()
                        .add_1("rotate-180");
                }
            }
            node = n.next_element_sibling();
        }

        //当前元素
        sibling.class_list().toggle("hidden");
    }
}