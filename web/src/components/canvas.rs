use gloo::{events::EventListener, utils::window, timers::callback::{Timeout, Interval}};
use std::{ops::Deref, cell::RefCell};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use yew::{html::ChildrenRenderer, prelude::*};
use yew_router::switch::_SwitchProps::render;

/*
 * Base on https://github.com/cxgreat2014/Yew-Canvas.rs
 */

#[function_component(Canvas)]
pub fn canvas<CanvasContext, T>(props: &Props<T>) -> Html
    where
        T: PartialEq + WithRander + Clone + 'static,
        CanvasContext: JsCast,
{
    let node_ref = NodeRef::default();
    let is_first_rander = use_state(|| true);
    let style = props.style.clone().unwrap_or(String::new());
    let display_size = use_state(|| (300, 150));

    let size_listen_enent_state = use_state(|| EventListener::new(&window(), "resize", |_| ()));

    {
        let node_ref = node_ref.clone();
        let display_size = display_size.clone();
        let rander = props.rander.clone();

        use_effect(move || {
            if let Some(canvas) = node_ref.cast::<HtmlCanvasElement>() {
                if *is_first_rander {
                    is_first_rander.set(false);
                    let canvas = canvas.clone();

                    display_size.set((canvas.client_width(), canvas.client_height()));

                    size_listen_enent_state.set(EventListener::new(
                        &window(),
                        "resize",
                        move |_| {
                            display_size.set((canvas.client_width(), canvas.client_height()));
                        },
                    ));
                }

                {
                    let rand = rander.clone();
                    let mut rand = rand.borrow_mut();
                    rand.rand(&canvas, true);
                    Interval::new(100, move || {
                        let mut rander = rander.borrow_mut();
                        rander.rand(&canvas, false);
                    }).forget();
                }
            }

            || ()
        });
    }

    let children = props
        .children
        .clone()
        .unwrap_or(ChildrenRenderer::default());

    html! {
    <canvas
        style={style}
        width={display_size.clone().deref().0.to_string()}
        height={display_size.deref().1.to_string()}
        onclick={props.onclick.clone()}
        ref={node_ref}
    >
        { for children.iter() }
    </ canvas>
    }
}

pub trait WithRander: Clone + PartialEq {
    fn rand(&mut self, canvas: &HtmlCanvasElement, full: bool);
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T: PartialEq> {
    pub rander: Rc<RefCell<T>>,
    pub children: Option<Children>,
    pub style: Option<String>,
    #[prop_or(Callback::from(|_| {}))]
    pub onclick: Callback<MouseEvent, ()>, 
}
