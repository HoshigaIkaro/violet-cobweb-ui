use yew::prelude::*;

use super::RecordingStateContext;

#[function_component(Recorder)]
pub fn recorder() -> Html {
    let recording_state = use_context::<RecordingStateContext>().expect("no recording state found");
    let recording = recording_state.inner;

    let toggle_recording = Callback::from(move |_| recording_state.dispatch(!recording));

    html! {
        <div class="recorder row">
            <div class="toggle">
                if recording {
                    <button type="button" onclick={toggle_recording}>{"Stop"}</button>
                } else {
                    <button type="button" onclick={toggle_recording}>{"Record"}</button>
                }
            </div>

            <div class="time">
                <p>{"Time elapsed placeholder"}</p>
            </div>

            <div class="mark">
                <button type="button">{"Mark"}</button>
            </div>
        </div>
    }
}
