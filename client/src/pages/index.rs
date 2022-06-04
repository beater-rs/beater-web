use gloo::{
    file::{futures::read_as_data_url, Blob},
    net::http::Request,
    storage::{LocalStorage, Storage},
};
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {
    let onsubmit = |e: FocusEvent| {
        e.prevent_default();

        let track_id = gloo::utils::document()
            .get_element_by_id("track_id")
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap()
            .value();

        let (username, password) = match (
            LocalStorage::get::<String>("spotify_username"),
            LocalStorage::get::<String>("spotify_password"),
        ) {
            (Ok(username), Ok(password)) => (username, password),
            _ => {
                gloo::utils::window()
                    .location()
                    .set_pathname("/login")
                    .unwrap();
                unreachable!();
            }
        };

        wasm_bindgen_futures::spawn_local(async move {
            let response = Request::get(&format!("/api/song/{track_id}"))
                .header(
                    "Authorization",
                    &format!(
                        "Basic {}",
                        gloo::utils::window()
                            .btoa(&format!("{username}:{password}"))
                            .unwrap()
                    ),
                )
                .send()
                .await
                .unwrap()
                .binary()
                .await
                .unwrap();

            let player_elem = gloo::utils::document()
                .get_element_by_id("player")
                .unwrap()
                .dyn_into::<web_sys::HtmlAudioElement>()
                .unwrap();
            player_elem.set_src(
                &read_as_data_url(&Blob::new_with_options(&*response, Some("audio/ogg")))
                    .await
                    .unwrap(),
            );
            let _ = player_elem.play().unwrap();
        });
    };

    html! {
        <>
            <form {onsubmit} class="field">
                <div class="control">
                    <label class="label" for="track_id">{ "Track ID" }</label>
                    <input class="input" id="track_id" type="text" />
                    <button class="button is-primary" type="submit">{ "Submit" }</button>
                </div>
            </form>
            <audio id="player" controls={true}></audio>
        </>
    }
}
