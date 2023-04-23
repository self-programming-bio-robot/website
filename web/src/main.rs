use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;
use zhdanov_website_core::page_repository::{PageLocalRepository, PageRepository};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlElement};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/pages/:name")]
    Page { name: String },
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
    }
}

#[function_component(ArticlePage)]
fn article_page(props: &ArticleProps) -> Html {
    let context = use_context::<Rc<Context>>().unwrap();
    let additional = use_state(String::new);
    let insert_text = {
        let additional = additional.clone();
        Callback::from(move |x: String| {
            let mut last_text = String::new();
            last_text.push_str(additional.as_str());
            last_text.push_str(x.as_str());
            additional.set(last_text);
        })
    };

    let clear_text = {
        let additional = additional.clone();
        Callback::from(move |_: ()| {
            additional.set("".into());
        })
    };

    if let Some(page_content) = context.database.get_page(&props.name[..]) {
        let content = page_content.content.clone();
        let links: Vec<String> = page_content.links.iter()
            .map(|x| x.to_string())
            .collect();
        html! {
            <>
                <ConsoleView 
                    text={content} 
                    after={additional.clone().to_string()} />
                <ConsoleInput 
                    links={links} 
                    on_error={insert_text}
                    on_submit={clear_text} />
            </>
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
    #[prop_or(AttrValue::from(""))]
    pub after: AttrValue,
}

#[function_component(ConsoleView)]
fn console_view(props: &ConsoleViewProps) -> Html {
    html! {
        <pre>
            {props.text.clone()}
            {props.after.clone()}
        </pre>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ConsoleInputProps {
    #[prop_or(vec![])]
    pub links: Vec<String>,
    pub on_error: Callback<String>,
    pub on_submit: Callback<()>,
}

#[function_component(ConsoleInput)]
fn console_input(props: &ConsoleInputProps) -> Html {
    let input_text = use_state(String::new); 
    let navigator = use_navigator().unwrap();
    let input_ref = use_node_ref();

    {
        let input_ref = input_ref.clone();
        use_effect_with_deps(|input_ref| {
            let input = input_ref
                .cast::<HtmlInputElement>()
                .expect("could not attach to input field");
            input.focus().unwrap();
        }, 
        input_ref);
    }

    let handle_submit = {
        let ConsoleInputProps{ links, on_error, on_submit } = props.clone();
        
        Callback::from({
            let input_text = input_text.clone();

            move |event: SubmitEvent| {
                event.prevent_default();
                if links.len() == 0 {
                    navigator.push(&Route::Page {
                        name: "main".into()
                    });        
                    input_text.set("".to_owned());
                    on_submit.emit(());
                } else {
                    let value = &input_text[..];
                    if let Ok(value) = value.parse::<usize>() {
                        if value < links.len() {
                            navigator.push(&Route::Page {
                                name: links[value].clone()
                            });
                            on_submit.emit(());
                        } else {
                            let message = format!("\n{}\nMax page number is {}\nTry again...", 
                                                  &value, links.len());
                            on_error.emit(message);
                        }
                    } else {
                        let message = format!("\n{}\nExpept number is in range 0..{}\nTry again...", 
                                                  &value, links.len()-1);
                        on_error.emit(message);
                    }
                    input_text.set("".to_owned());
                }
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
        <p>
            <form onsubmit={handle_submit}>
                <input ref={input_ref} oninput={handle_input} value={input_text.clone().to_string()} />
            </form>
        </p>
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
    let database = use_memo(|_| Context {
        database: Rc::new(PageLocalRepository::default())
    }, ());
    
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
