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
            let node = elem
                .parent_node()
                .unwrap()
                .parent_element()
                .unwrap()
                .first_child()
                .unwrap();
            let mut i = 0;
            while (true) {
                if let Some(s) = node.next_sibling() {
                    if s.node_type() == 1 && !elem.is_same_node(Some(&s)) {}
                } else {
                    break;
                }
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
                            <li class="bg-gray-300">{"文章分类"}</li>
                            <li>{"文章录入"}</li>
                            <li>{"文章审核"}</li>
                        </ul>
                    </li>
                    <li class="px-3 py-2  w-1/12 bg-gray-600 border-b-2 cursor-pointer">
                        <p  onclick = &onclick class="t2">{"用户管理"}</p>
                        <ul class="duration-300 hidden">
                            <li>{"前端用户"}</li>
                            <li>{"后台用户"}</li>
                        </ul>
                    </li>
                </ul>
            </div>
        }
    }
}
