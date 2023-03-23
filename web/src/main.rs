use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;
use zhdanov_website_core::page_repository::{PageLocalRepository, PageRepository};

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

#[derive(Properties, PartialEq)]
pub struct ArticleProps {
    #[prop_or(AttrValue::from("main"))]
    pub name: AttrValue,
}

fn router(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::NotFound => html! { <NotFoundPage /> },
        Route::Page { name } => html! { 
            <ArticlePage name={name} /> 
        },
    }
}

#[function_component(HomePage)]
fn home_page() -> Html {
    let context = use_context::<Rc<Context>>().unwrap();
    let page_content = context.database.get_page("main").unwrap();
    html! {
        <pre>{page_content.content.clone()}</pre>
    }
}

#[function_component(ArticlePage)]
fn article_page(props: &ArticleProps) -> Html {
    let context = use_context::<Rc<Context>>().unwrap();
    if let Some(page_content) = context.database.get_page(&props.name[..]) {
        html! {
            <pre>{page_content.content.clone()}</pre>
        }
    } else {
        html! {
            <Redirect<Route> to={Route::NotFound}/>
        }
    }
}

#[function_component(NotFoundPage)]
fn not_found_page() -> Html {
    let context = use_context::<Rc<Context>>().unwrap();
    let page_content = context.database.get_page("404").unwrap();
    html! {
        <pre>{page_content.content.clone()}</pre>
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
