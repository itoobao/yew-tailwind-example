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
        }
        true
    }

    fn view(&self) -> Html {
        let onclick = Callback::from(|ev: MouseEvent| {
            let elem = ev
                .target()
                .expect("no event target")
                .dyn_into::<HtmlElement>()
                .expect("no element");
            //elem.class_list().add_1("animate-spin");
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
                          <button class="bg-blue-500 text-sm py-3 text-white rounded w-2/4" onclick=&onclick>
                          <svg class="animate-spin h-5 w-5 mr-3" viewBox="0 0 24 24"></svg>
        {"登录"} </button>
                      </div>
                  </section>
              }
    }
}
