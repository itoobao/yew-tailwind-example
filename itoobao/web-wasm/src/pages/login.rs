use yew::prelude::*;

use crate::components::ErrorTips;
use crate::error::Error;
use crate::services::set_token;
use crate::types::LoginInfo;
pub struct Login {
    login_info: LoginInfo,
    error: Option<Error>,
    link: ComponentLink<Self>,
}
pub enum Msg {
    SetEmail(String),
    SetPassword(String),
    Submit,
}
impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            login_info: LoginInfo::default(),
            error: None,
            link,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.error = None;
        match msg {
            Msg::SetEmail(email) => self.login_info.username = email,
            Msg::SetPassword(password) => self.login_info.password = password,
            Msg::Submit => {
                //self.error = Some(Error::BusinessError("用户名或密码错误".to_string()));
                set_token(Some(self.login_info.username.clone()));
                //跳转到首页
            }
        }
        true
    }

    fn view(&self) -> Html {
        let oninput_email = self.link.callback(|ev: InputData| Msg::SetEmail(ev.value));
        let oninput_password = self
            .link
            .callback(|ev: InputData| Msg::SetPassword(ev.value));
        let onclick = self.link.callback(|ev: MouseEvent| Msg::Submit);
        html! {
            <div class="flex items-center justify-center h-screen px-6 bg-gray-300">
                <div class="w-full max-w-sm p-6 bg-white rounded-md shadow-md">
                    <div class="flex items-center justify-center">
                        <svg class="w-10 h-10" viewBox="0 0 512 512" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path d="M364.61 390.213C304.625 450.196 207.37 450.196 147.386 390.213C117.394 360.22 102.398 320.911 102.398 281.6C102.398 242.291 117.394 202.981 147.386 172.989C147.386 230.4 153.6 281.6 230.4 307.2C230.4 256 256 102.4 294.4 76.7999C320 128 334.618 142.997 364.608 172.989C394.601 202.981 409.597 242.291 409.597 281.6C409.597 320.911 394.601 360.22 364.61 390.213Z"
                            fill="#4C51BF" stroke="#4C51BF" stroke-width="2" stroke-linecap="round"
                            stroke-linejoin="round">
                            </path>
                            <path d="M201.694 387.105C231.686 417.098 280.312 417.098 310.305 387.105C325.301 372.109 332.8 352.456 332.8 332.8C332.8 313.144 325.301 293.491 310.305 278.495C295.309 263.498 288 256 275.2 230.4C256 243.2 243.201 320 243.201 345.6C201.694 345.6 179.2 332.8 179.2 332.8C179.2 352.456 186.698 372.109 201.694 387.105Z"
                            fill="white">
                            </path>
                        </svg>
                        <span class="text-2xl font-semibold text-gray-700">
                            {"租号玩活动管理"}
                        </span>
                    </div>

                    <ErrorTips error=self.error.clone()/>

                    <div class="mt-4">
                        <div>
                            <span class="text-sm text-gray-700">{"Email"}</span>
                            <input type="email" class="mt-1" placeholder="请输入用户名" oninput = oninput_email/>
                        </div>
                        <div class="mt-3">
                            <span class="text-sm text-gray-700">{"Password"}</span>
                            <input type="password" class="mt-1" placeholder="请输入密码" oninput = oninput_password/>
                        </div>
                        <div class="mt-6">
                            <button type="button" class="w-full px-4 py-2 text-sm text-center text-white bg-indigo-600 rounded-md focus:outline-none hover:bg-indigo-500" onclick=onclick>{"Sign in"}</button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
