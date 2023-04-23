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
    let navigator = use_navigator().unwrap();
    let input_text = use_state(String::new);
    let input_ref = use_node_ref();
    let output_ref = use_node_ref();

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

    if let Some(page_content) = context.database.get_page(&props.name[..]) {

        let handle_submit = Callback::from({
            let links: Vec<String> = page_content.links.iter()
                .map(|x| x.to_string()).collect();
            let input_text = input_text.clone();
            let output_ref = output_ref.clone();

            move |event: SubmitEvent| {
                event.prevent_default();
                if links.len() == 0 {
                    navigator.push(&Route::Page {
                        name: "main".into()
                    });        
                    input_text.set("".to_owned());
                } else {
                    let value = &input_text[..];
                    if let Ok(value) = value.parse::<usize>() {
                        if value < links.len() {
                            navigator.push(&Route::Page {
                                name: links[value].clone()
                            });
                        } else {
                            let message = format!("\n{}\nMax page number is {}\nTry again...", 
                                                  &value, links.len());
                            add_text_to_console(output_ref.clone(), message.as_str());
                        }
                    } else {
                        let message = format!("\n{}\nExpept number is in range 0..{}\nTry again...", 
                                                  &value, links.len()-1);
                        add_text_to_console(output_ref.clone(), message.as_str());
                    }
                    input_text.set("".to_owned());
                }
            }
        });

        let handle_input = {
            let input_text = input_text.clone();
            Callback::from(move |event: InputEvent| {
                let input = event.target();
                let input = input.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                let input = input.unwrap();

                input_text.set(input.value());
            })
        };
        let text = &input_text[..].to_owned();
        html! {
            <>
                <pre ref={output_ref}>{page_content.content.clone()}</pre>
                <p>
                    <form onsubmit={handle_submit}>
                        <input ref={input_ref} oninput={handle_input} value={text.clone()} />
                    </form>
                </p>
            </>
        }
    } else {
        html! {
            <Redirect<Route> to={Route::NotFound}/>
        }
    }
}

fn add_text_to_console(node_ref: NodeRef, text: &str) {
    let console = node_ref
        .cast::<HtmlElement>()
        .expect("could not attach to element");
    let mut content = console.text_content().unwrap();
    content.push_str(text);
    console.set_text_content(Some(&content[..]));
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
