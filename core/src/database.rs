use crate::pages::Page;

pub fn pages() -> Vec<Page<'static>> {
    vec![
        Page {
            id: "main",
            title: "Zhdanov Dev Website".to_string(),
            content: include_str!("../pages/main.txt").to_string(),
            links: vec!["about_me", "cv"],
        },
        Page {
            id: "404",
            title: "Page not found".to_string(),
            content: include_str!("../pages/404.txt").to_string(),
            links: vec![]
        },
    ]
}
