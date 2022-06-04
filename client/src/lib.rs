#![feature(explicit_generic_args_with_impl_trait)]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod router;
use router::{switch, Routes};

mod components;
mod pages;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Routes> render={Switch::render(switch)} />
            </BrowserRouter>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    yew::start_app_in_element::<App>(gloo::utils::document().get_element_by_id("app").unwrap());
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
