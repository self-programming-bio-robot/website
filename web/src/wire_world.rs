
use web_sys::MouseEvent;
use yew::{function_component, Html, html, Callback, use_effect};
// use zhdanov_wire_world::GamePlugin;

#[function_component(WireWorld)]
pub fn wire_world() -> Html {
    use_effect(|| {
        // GamePlugin::start();
    });

    html! {
        <div id="fullsize">
            <canvas id="render" oncontextmenu={Callback::from(|x: MouseEvent| x.prevent_default())}>
            </canvas>
        </div>
    }
}
