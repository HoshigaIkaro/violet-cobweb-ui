use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod example;
mod player;
mod recorder;

use player::Player;
use recorder::Recorder;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct RecordingState {
    pub inner: bool,
    pub available_for_playback: bool,
    pub file_name: String,
}

impl Reducible for RecordingState {
    type Action = bool;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let available_for_playback = action && !self.available_for_playback;

        RecordingState {
            inner: action,
            available_for_playback,
            file_name: self.file_name.clone(),
        }
        .into()
    }
}

pub type RecordingStateContext = UseReducerHandle<RecordingState>;

#[function_component(App)]
pub fn app() -> Html {
    let recording_state = use_reducer(RecordingState::default);

    html! {
        <main class="container">
            <ContextProvider<RecordingStateContext> context={recording_state}>
                <Player />
                <Recorder />
            </ContextProvider<RecordingStateContext>>
        </main>
    }
}
