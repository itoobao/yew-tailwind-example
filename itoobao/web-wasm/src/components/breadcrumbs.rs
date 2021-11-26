use yew::prelude::*;

pub struct BreadCrumbs {}

impl Component for BreadCrumbs {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <nav class="bg-grey-light rounded font-sans w-full">
                <ol class="list-reset flex text-grey-dark">
                    <li><a href="#" class="font-bold">{"Home"}</a></li>
                    <li><span class="mx-2">{"/"}</span></li>
                    <li><a href="#" class="font-bold">{"Library"}</a></li>
                    <li><span class="mx-2">{"/"}</span></li>
                    <li>{"Data"}</li>
                </ol>
            </nav>
        }
    }
}
