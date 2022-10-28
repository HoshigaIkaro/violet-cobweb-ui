use super::RecordingStateContext;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component(Player)]
pub fn player() -> Html {
    let recording_state = use_context::<RecordingStateContext>().expect("no recording state found");
    let recording = recording_state.inner;

    html! {
        <div class="player">
            <div class="row">
                if recording {
                    <p>{"Recording in progress."}</p>
                } else {
                    <p>{"Placeholder"}</p>
                }
            </div>
        </div>
    }
}
