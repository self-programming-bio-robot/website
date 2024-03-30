use std::rc::Rc;

use futures_util::StreamExt;
use gloo::console::{console, console_dbg};
use gloo::net::http::Request;
use wasm_bindgen::JsCast;
use wasm_streams::ReadableStream;
use web_sys::HtmlInputElement;
use web_sys::js_sys::Uint8Array;
use yew::prelude::*;
use yew_hooks::{use_list, use_update};
use yew_router::prelude::*;

use wire_world::WireWorld;
use zhdanov_website_core::page_repository::{PageLocalRepository, PageRepository};

pub mod wire_world;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/pages/:name")]
    Page { name: String },
    #[at("/pages/wire-world")]
    WireWorld,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ArticleProps {
    #[prop_or(AttrValue::from("main"))]
    pub name: AttrValue,
}

fn router(route: Route) -> Html {
    match route {
        Route::Home => html! { 
            <ArticlePage name="main" /> 
        },
        Route::NotFound => html! { 
            <ArticlePage name="404" />
        },
        Route::Page { name } => html! {
            <ArticlePage name={name} />
        },
        Route::WireWorld => html! { 
            <WireWorld />
        },
    }
}

#[function_component(ArticlePage)]
fn article_page(props: &ArticleProps) -> Html {
    let context = use_context::<Rc<Context>>().unwrap();
    let messages = use_list(Vec::<Message>::new());
    let assistant_response = use_mut_ref(String::new);
    let is_generating = use_mut_ref(|| false);
    let update = use_update();

    if let Some(page_content) = context.database.get_page(&props.name[..]) {

        let insert_text = {
            let messages = messages.clone();
            Callback::from(move |x: String| {
                messages.push(Message {
                    content: x,
                    is_assistant: true,
                });
            })
        };

        let add_user_message = {
            let messages = messages.clone();
            Callback::from(move |message: String| {
                messages.push(Message {
                    content: message,
                    is_assistant: false,
                });
            })
        };

        let start_ai_stream = {
            let is_generating = is_generating.clone();
            let assistant_response = assistant_response.clone();
            let update = update.clone();

            Callback::from(move |()| {
                let mut assistant_response = assistant_response.borrow_mut();
                let mut is_generating = is_generating.borrow_mut();

                console_dbg!("Stream started");
                *is_generating = true;
                assistant_response.clear();
                update();
            })
        };

        let update_ai_stream = {
            let assistant_response = assistant_response.clone();
            let update = update.clone();

            Callback::from(move |message: String| {
                let mut assistant_response = assistant_response.borrow_mut();
                console_dbg!("Stream updated", message);
                assistant_response.push_str(message.clone().as_str());
                update();
            })
        };

        let complete_ai_stream = {
            let messages = messages.clone();
            let is_generating = is_generating.clone();
            let assistant_response = assistant_response.clone();

            Callback::from(move |()| {
                let mut is_generating = is_generating.borrow_mut();

                console_dbg!("Stream completed");
                *is_generating = false;
                messages.push(Message {
                    content: assistant_response.borrow().to_string(),
                    is_assistant: true,
                });
            })
        };

        let messages_string = messages.current().iter()
            .map(|message| {
                if message.is_assistant {
                    format!("Assistant: {}", message.content)
                } else {
                    format!("You: {}", message.content)
                }
            })
            .collect::<Vec<String>>()
            .join("\n\n");

        let messages_string = if *is_generating.borrow() {
            format!("{}\n\nAssistant: {}", messages_string, assistant_response.borrow())
        } else {
            messages_string
        };


        let content = page_content.content.clone();
        let links: Vec<String> = page_content.links.iter()
            .map(|x| x.to_string())
            .collect();
        let messages_string = format!("{}\n\n{}", content, messages_string);

        html! {
            <div class="wrapper">
                <ConsoleView
                    text={messages_string} />
                <ConsoleInput
                    on_error={insert_text}
                        on_submit={add_user_message}
                        on_start_stream={start_ai_stream}
                        on_update_stream={update_ai_stream}
                        on_complete_stream={complete_ai_stream}
                />
            </div>
        }
    } else {
        html! {
            <Redirect<Route> to={Route::NotFound}/>
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ConsoleViewProps {
    #[prop_or(AttrValue::from(""))]
    pub text: AttrValue,
}

#[function_component(ConsoleView)]
fn console_view(props: &ConsoleViewProps) -> Html {
    html! {
        <pre>
            {props.text.clone()}
        </pre>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ConsoleInputProps {
    pub on_error: Callback<String>,
    pub on_submit: Callback<String>,
    pub on_start_stream: Callback<()>,
    pub on_update_stream: Callback<String>,
    pub on_complete_stream: Callback<()>,
}

#[function_component(ConsoleInput)]
fn console_input(props: &ConsoleInputProps) -> Html {
    let input_text = use_state(String::new);
    let input_ref = use_node_ref();
    {
        let input_ref = input_ref.clone();
        use_effect_with(input_ref, |input_ref| {
            let input = input_ref
                .cast::<HtmlInputElement>()
                .expect("could not attach to input field");
            input.focus().unwrap();
        });
    }

    let handle_submit = {
        let ConsoleInputProps {
            on_error,
            on_submit,
            on_start_stream,
            on_update_stream,
            on_complete_stream,
        } = props.clone();

        Callback::from({
            let input_text = input_text.clone();

            move |event: SubmitEvent| {
                event.prevent_default();
                on_submit.emit(input_text.clone().to_string());

                wasm_bindgen_futures::spawn_local({
                    let input_text = input_text.clone();
                    let on_start_stream = on_start_stream.clone();
                    let on_complete_stream = on_complete_stream.clone();
                    let on_update_stream = on_update_stream.clone();

                    async move {
                        on_start_stream.emit(());
                        let raw = Request::post("api/answer")
                            .body(input_text.clone().to_string())
                            .unwrap()
                            .send()
                            .await
                            .unwrap()
                            .body()
                            .unwrap();
                        let body = ReadableStream::from_raw(raw);
                        let mut stream = body.into_stream();

                        while let Some(Ok(chunk)) = stream.next().await {
                            let uint8_array: Uint8Array = chunk.into();
                            let vec: Vec<u8> = uint8_array.to_vec();

                            if let Ok(chunk) = String::from_utf8(vec) {
                                on_update_stream.emit(chunk);
                            }
                        }
                        on_complete_stream.emit(());
                    }
                });

                input_text.set("".to_owned());
            }
        })
    };

    let handle_input = {
        let input_text = input_text.clone();
        Callback::from(move |event: InputEvent| {
            let input = event.target();
            let input = input.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let input = input.unwrap();

            input_text.set(input.value());
        })
    };

    html! {
        <p class="input">
            <form onsubmit={handle_submit}>
                <label>{">\u{00a0}"}</label>
                <input 
                    ref={input_ref} 
                    oninput={handle_input} 
                    value={input_text.clone().to_string()} />
            </form>
        </p>
    }
}

#[derive(Clone)]
pub struct Message {
    pub content: String,
    pub is_assistant: bool,
}

#[derive(Clone)]
pub struct Context {
    pub database: Rc<PageLocalRepository<'static>>,
}

impl PartialEq for Context {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[function_component(Main)]
fn app() -> Html {
    let database = use_memo((), |_| Context {
        database: Rc::new(PageLocalRepository::default())
    });

    html! {
        <ContextProvider<Rc<Context>> context={database}>
            <BrowserRouter>
                <Switch<Route> render={router} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </ContextProvider<Rc<Context>>>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
