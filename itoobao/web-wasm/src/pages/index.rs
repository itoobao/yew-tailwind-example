use wasm_bindgen::JsCast;
use yew::{prelude::*, web_sys::HtmlElement};

pub struct Index {}

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let onclick = Callback::from(|ev: MouseEvent| {
            let elem = ev
                .target()
                .expect("no event target")
                .dyn_into::<HtmlElement>()
                .expect("no element");
            elem.class_list().add_1("py-10");
        });
        html! {
            <div class="text-gray-500">
                <div onclick=onclick> {"this is index"}</div>
            </div>
        }
    }
}
