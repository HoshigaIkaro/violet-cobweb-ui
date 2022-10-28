use super::RecordingStateContext;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use yew::prelude::*;
use yew_hooks::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[function_component(Player)]
pub fn player() -> Html {
    let recording_state = use_context::<RecordingStateContext>().expect("no recording state found");

    let node_ref = use_node_ref();
    let file_path = "".to_string();
    let audio = use_media(node_ref.clone(), file_path);
    log("Here");

    let onplay = {
        let audio = audio.clone();
        Callback::from(move |_| audio.play())
    };

    let onpause = {
        let audio = audio.clone();
        Callback::from(move |_| audio.pause())
    };

    let onmute = {
        let audio = audio.clone();
        Callback::from(move |_| audio.mute())
    };

    let onunmute = {
        let audio = audio.clone();
        Callback::from(move |_| audio.unmute())
    };

    let onvol = {
        let audio = audio.clone();
        Callback::from(move |_| audio.set_volume(0.5))
    };
    let onseek = {
        let audio = audio.clone();
        Callback::from(move |_| audio.seek(*audio.time + 5.0))
    };

    html! {
        <div class="player col">
            <audio ref={node_ref} />

            <div class="info">
                    <p>{ " Duration: " } { *audio.duration }</p>
                    <p>{ " Time: " } { *audio.time }</p>
                    <p class="volume">
                        if *audio.muted {
                            { "Muted" }
                        } else {
                            { " Volume: " } { *audio.volume * 100.0 } { "%" }

                        }
                    </p>
            </div>

            <div class="controls">
                <div class="volume">
                    if *audio.muted {
                        <button onclick={onunmute} disabled={!*audio.muted}>{ "Unmute" }</button>
                    } else {
                        <button onclick={onmute} disabled={*audio.muted}>{ "Mute" }</button>
                    }

                    <button onclick={onvol}>{ "Volume: 50%" }</button>
                </div>

                <div class="toggle">
                    if *audio.playing {
                        <button onclick={onpause} disabled={*audio.paused}>{ "Pause" }</button>
                    } else {
                        <button onclick={onplay} disabled={*audio.playing}>{ "Play" }</button>
                    }
                </div>

                <div class="playback col">
                    <p>{ "Seek 5s:" }</p>
                    <div class="seek">
                        <button>{ "←" }</button>
                        <button onclick={onseek}>{ "→" }</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
