use crate::pages;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Clone, PartialEq, Eq, Copy)]
pub enum Routes {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Routes) -> Html {
    use Routes::*;
    match *route {
        Home => {
            html! {
                <pages::index::Index />
            }
        }
        Login => {
            html! {
                <pages::login::Login />
            }
        }
        NotFound => {
            html! {
                <pages::not_found::NotFound />
            }
        }
    }
}
