use yew::prelude::*;

use crate::error::Error;

pub struct ErrorTips {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub error: Option<Error>,
}

pub enum Msg {
    Close,
}
impl Component for ErrorTips {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.props = _props;
        true
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Close => self.props.error = None,
        }
        true
    }

    fn view(&self) -> Html {
        if let Some(e) = &self.props.error {
            let close = self.link.callback(|ev: MouseEvent| Msg::Close);
            html! {
                <div class="my-3 block text-sm text-left text-white bg-red-500 h-12 flex items-center rounded-md relative">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="w-6 h-6 mx-2 stroke-current">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                        ></path>
                    </svg>
                    {e}
                    <button class="absolute bg-transparent text-2xl font-semibold leading-none right-0 top-0 mt-2 mr-6 outline-none focus:outline-none" onclick=&close>
                        <span>{"x"}</span>
                    </button>
                </div>
            }
        } else {
            html! {}
        }
    }
}
