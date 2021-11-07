use log::debug;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
pub struct Menu {
    link: ComponentLink<Self>,
}

impl Component for Menu {
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
        let onclick = self.link.callback(|ev: MouseEvent| {
            let elem = ev
                .target()
                .expect("no target")
                .dyn_into::<HtmlElement>()
                .expect("no element");
            //父节点的父节点的第一个元素
            let mut node = elem
                .parent_node()
                .unwrap()
                .parent_element()
                .unwrap()
                .first_element_child();
            //当前元素的下一个兄弟元素
            let sibling_elem = elem.next_element_sibling().unwrap();
            while (node.is_some()) {
                //子菜单
                let n = &node.unwrap();
                if let Some(m) = n.children().get_with_index(1) {
                    if !m.is_same_node(Some(&sibling_elem)) {
                        m.class_list().add_1("hidden");
                    }
                }
                node = n.next_element_sibling();
            }

            //子菜单处理
            if let Some(sibling) = elem.next_element_sibling() {
                sibling.class_list().toggle("hidden");
            }
        });

        html! {
            <div>
                <ul class="list-none">
                    <li class="px-3 py-2 w-1/12 bg-gray-600 border-b-2 cursor-pointer">{"Dash"}</li>
                    <li class="px-3 py-2  w-1/12 bg-gray-600 border-b-2 cursor-pointer">
                        <p  onclick = &onclick>{"文章管理"}</p>
                        <ul class="duration-300 hidden">
                            <li class="hover:bg-gray-500">{"文章分类"}</li>
                            <li class="hover:bg-gray-500">{"文章录入"}</li>
                            <li class="hover:bg-gray-500">{"文章审核"}</li>
                        </ul>
                    </li>
                    <li class="px-3 py-2  w-1/12 bg-gray-600 border-b-2 cursor-pointer">
                        <p  onclick = &onclick class="t2">{"用户管理"}</p>
                        <ul class="duration-300 hidden">
                            <li class="hover:bg-gray-500">{"前端用户"}</li>
                            <li class="hover:bg-gray-500">{"后台用户"}</li>
                        </ul>
                    </li>
                </ul>
            </div>
        }
    }
}
