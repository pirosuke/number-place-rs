#![recursion_limit="512"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;
use components::game;

struct Model {
}

enum Msg {
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div style="height: 100%;">
                <svg
                    id="number-place-game"
                    style="width:100%; height: 80%;" >
                    <game::Game 
                        x=10
                        y=10 />
                </svg>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}