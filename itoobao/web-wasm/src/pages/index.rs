use log::debug;
use wasm_bindgen::JsCast;
use yew::{prelude::*, web_sys::HtmlElement};

use crate::types::LoginInfo;

pub struct Index {
    request: LoginInfo,
    link: ComponentLink<Self>,
}

pub enum Msg {
    UserNameMsg(String),
    PasswordMsg(String),
    Request(HtmlElement),
}
impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            request: LoginInfo::default(),
            link,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UserNameMsg(username) => {
                self.request.username = username;
            }
            Msg::PasswordMsg(pwd) => {
                self.request.password = pwd;
            }
            Msg::Request(elem) => {
                let first_child = elem.first_element_child().expect("no child");
                first_child.class_list().add_1("hidden");
                elem.class_list()
                    .remove_2("bg-gray-300", "cursor-not-allowed");
                elem.class_list().add_1("bg-blue-500");
                elem.remove_attribute("disabled");
                debug!("sleep 3");
            }
        }
        true
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|ev: MouseEvent| {
            debug!("button click");
            let elem = ev
                .target()
                .expect("no event target")
                .dyn_into::<HtmlElement>()
                .expect("no element");
            let first_child = elem.first_element_child().expect("no child");
            first_child.class_list().remove_1("hidden");
            elem.class_list().remove_1("bg-blue-500");
            elem.class_list().add_2("bg-gray-300", "cursor-not-allowed");
            elem.set_attribute("disabled", "");
            Msg::Request(elem)
        });
        let oninput_username = self
            .link
            .callback(|ev: InputData| Msg::UserNameMsg(ev.value));
        html! {
            <section class="bg-white rounded-md shadow-md mx-auto max-w-md items-center p-5">
                <div class="mx-5">
                    <p class="text-gray-300 hover:text-gray-600">{"用户名"}</p>
                    <input class="input placeholder-gray-300 placeholder-opacity-100" type="text" required=true placeholder="用户名" value = self.request.username.clone() oninput=oninput_username/>
                </div>
                <div class="m-5">
                    <p class="text-gray-300 hover:text-gray-600">{"密码"}</p>
                    <input class="input placeholder-gray-300 placeholder-opacity-100" type="password" placeholder="密码" value = self.request.password.clone()/>
                </div>
                <div class="m-4">

          <button type="button" class="inline-flex items-center px-4 py-2 border border-transparent text-base leading-6 font-medium rounded-md text-white bg-blue-500 transition ease-in-out duration-150 rounded w-2/4"
          onclick=&onclick>
              <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white hidden" xmlns="http://www.w3.org/2000/svg"
              fill="none" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor"
                  stroke-width="4">
                  </circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
                  </path>
              </svg>
              {"登录"}
          </button>

                </div>
            </section>
        }
    }
}
