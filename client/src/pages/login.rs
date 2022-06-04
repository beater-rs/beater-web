use gloo::storage::{LocalStorage, Storage};
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    let onsubmit = |e: FocusEvent| {
        e.prevent_default();

        let username = gloo::utils::document()
            .get_element_by_id("username")
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap()
            .value();
        let password = gloo::utils::document()
            .get_element_by_id("password")
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap()
            .value();

        LocalStorage::set("spotify_username", username).unwrap();
        LocalStorage::set("spotify_password", password).unwrap();

        gloo::utils::window().location().set_pathname("/").unwrap();
    };

    html! {
        <>
            <h1>{ "Login" }</h1>
            <form {onsubmit} class="field">
                <label class="label" for="username">{ "Username" }</label>
                <input class="input" id="username" type="text" />
                <label class="label" for="password">{ "Password" }</label>
                <input class="input" id="password" type="password" />
                <button class="button is-primary" type="submit">{ "Submit" }</button>
            </form>
        </>
    }
}
