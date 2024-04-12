use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use futures_util::StreamExt;
use gloo::net::http::Request;
use wasm_bindgen::{JsCast};
use wasm_streams::ReadableStream;
use web_sys::HtmlElement;
use web_sys::js_sys::Uint8Array;
use yew::prelude::*;
use yew_hooks::{use_event_with_window, use_list, use_update};
use yew_router::prelude::*;

use wire_world::WireWorld;
use zhdanov_website_core::dto::message::Message;
use zhdanov_website_core::dto::question::UserQuestion;
use zhdanov_website_core::page_repository::{PageLocalRepository, PageRepository};
use zhdanov_website_core::string_utils::{split_line_by_limit};

pub mod wire_world;

const ASSISTANT_NAME: &str = "Warton";
const USER_NAME: &str = "You";

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
    let navigator = use_navigator().unwrap();
    let context = use_context::<Rc<Context>>().unwrap();
    let messages = use_list(Vec::<Message>::new());
    let assistant_response = use_mut_ref(String::new);
    let is_generating = use_mut_ref(|| false);
    let update = use_update();
    let pre_ref = use_node_ref();

    fn scroll_bottom(pre_ref: NodeRef) {
        if let Some(node) = pre_ref.get() {
            if let Some(element) = node.dyn_ref::<HtmlElement>() {
                element.set_scroll_top(element.scroll_height());
            }
        }
    }

    fn prepare_string(text: &String) -> String {
        text.lines()
            .flat_map(|line| {
                split_line_by_limit(line, 80)
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    if let Some(page_content) = context.database.get_page(&props.name[..]) {
        let insert_text = {
            let messages = messages.clone();
            Callback::from(move |x: String| {
                messages.push(Message {
                    content: x,
                    is_assistant: true,
                    is_response: false,
                    is_question: false,
                    topic: None,
                });
            })
        };

        let add_user_message = {
            let messages = messages.clone();
            Callback::from(move |message: String| {
                messages.push(Message {
                    content: prepare_string(&format!("{}: {}", USER_NAME, message)),
                    is_assistant: false,
                    is_response: false,
                    is_question: true,
                    topic: None,
                });
            })
        };

        let start_ai_stream = {
            let is_generating = is_generating.clone();
            let assistant_response = assistant_response.clone();
            let update = update.clone();

            Callback::from(move |()| {
                let mut assistant_response: RefMut<String> = assistant_response.borrow_mut();
                let mut is_generating = is_generating.borrow_mut();

                *is_generating = true;
                assistant_response.clear();
                assistant_response.push_str(ASSISTANT_NAME);
                assistant_response.push_str(": ");

                update();
            })
        };

        let update_ai_stream = {
            let assistant_response = assistant_response.clone();
            let update = update.clone();
            let pre_ref = pre_ref.clone();
            let buffer = RefCell::new(ASSISTANT_NAME.to_owned() + ": ");

            Callback::from(move |chunk: String| {
                let mut assistant_response: RefMut<String> = assistant_response.borrow_mut();
                let mut buffer = buffer.borrow_mut();
                buffer.push_str(chunk.clone().as_str());
                *assistant_response = prepare_string(&buffer);

                update();

                scroll_bottom(pre_ref.clone());
            })
        };

        let complete_ai_stream = {
            let messages = messages.clone();
            let is_generating = is_generating.clone();
            let assistant_response = assistant_response.clone();
            let pre_ref = pre_ref.clone();
            let navigator = navigator.clone();

            Callback::from(move |(topic, is_question)| {
                let navigator = navigator.clone();
                let mut is_generating = is_generating.borrow_mut();
                let assistant_response: &String = &assistant_response.borrow();
                *is_generating = false;

                messages.push(Message {
                    content: prepare_string(assistant_response),
                    is_assistant: true,
                    is_response: true,
                    is_question,
                    topic: Some(topic),
                });

                scroll_bottom(pre_ref.clone());
            })
        };

        let messages_string = messages.current().iter()
            .map(|message| {
                message.content.clone()
            })
            .collect::<Vec<String>>()
            .join("\n\n");

        let messages_string = if *is_generating.borrow() {
            format!("{}\n\n{}", messages_string, assistant_response.borrow())
        } else {
            messages_string
        };

        let content = page_content.content.clone();
        let messages_string = format!("{}\n\n{}", content, messages_string);

        html! {
            <div class="screen">
                <div class="scanline"></div>
                <div ref={pre_ref.clone()} class="view">
                    <ConsoleView
                        text={messages_string}
                        is_generating={*is_generating.borrow()}
                    />
                    <ConsoleInput
                        page={props.name.to_string()}
                        is_generating={*is_generating.borrow()}
                        messages={messages.current().clone()}
                        on_error={insert_text}
                        on_submit={add_user_message}
                        on_start_stream={start_ai_stream}
                        on_update_stream={update_ai_stream}
                        on_complete_stream={complete_ai_stream}
                    />
                </div>
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
    pub is_generating: bool,
}

#[function_component(ConsoleView)]
fn console_view(props: &ConsoleViewProps) -> Html {
    html! {
        <pre>
            {props.text.clone()}<span class="cursor" data-generating={props.is_generating.to_string()}>{"■"}</span>{"\n\n"}
        </pre>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ConsoleInputProps {
    pub page: String,
    pub is_generating: bool,
    pub messages: Vec<Message>,
    pub on_error: Callback<String>,
    pub on_submit: Callback<String>,
    pub on_start_stream: Callback<()>,
    pub on_update_stream: Callback<String>,
    pub on_complete_stream: Callback<(String, bool)>,
}

#[function_component(ConsoleInput)]
fn console_input(props: &ConsoleInputProps) -> Html {
    let input_text = use_state(String::new);

    let handle_submit = {
        let ConsoleInputProps {
            page,
            is_generating: _,
            messages,
            on_error: _,
            on_submit,
            on_start_stream,
            on_update_stream,
            on_complete_stream,
        } = props.clone();

        Callback::from({
            let input_text = input_text.clone();
            
            move |()| {
                on_submit.emit(input_text.clone().to_string());

                wasm_bindgen_futures::spawn_local({
                    let input_text = input_text.clone();
                    let on_start_stream = on_start_stream.clone();
                    let on_complete_stream = on_complete_stream.clone();
                    let on_update_stream = on_update_stream.clone();
                    let page = page.clone();
                    let messages = messages.clone();

                    async move {
                        on_start_stream.emit(());

                        let request = UserQuestion {
                            question: input_text.to_string(),
                            from_page: page,
                            messages
                        };
                        let request = serde_json::to_string(&request).unwrap();
                        let response = Request::post("api/answer")
                            .header("Content-Type", "application/json")
                            .body(request)
                            .unwrap()
                            .send()
                            .await
                            .unwrap();
                        let is_question = response.headers().get("x-is-question").unwrap().as_str() == "true";
                        let topic = response.headers().get("x-topic").unwrap();
                        let raw = response
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
                        on_complete_stream.emit((topic, is_question));
                    }
                });

                input_text.set("".to_owned());
            }
        })
    };

    {
        let input_text = input_text.clone();
        let is_generating = props.is_generating.clone();
        use_event_with_window("keypress", move |e: KeyboardEvent| {
            let input_text = input_text.clone();
            if is_generating {
                return;
            }
            input_text.set(input_text.to_string() + e.key().as_str());
        });
    }
    {
        let input_text = input_text.clone();
        let handle_submit = handle_submit.clone();
        let is_generating = props.is_generating.clone();

        use_event_with_window("keydown", move |e: KeyboardEvent| {
            if is_generating {
                return;
            }
            match e.code().as_str() {
                "Backspace" => {
                    let mut text = input_text.to_string();
                    text.pop();
                    input_text.set(text);
                }
                "Enter" => {
                    e.prevent_default();
                    handle_submit.emit(())
                }
                _ => {}
            }
        });
    }

    html! {
        <span class="input" data-locked={props.is_generating.to_string()}>
            {input_text.to_string()}
        </span>
    }
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
