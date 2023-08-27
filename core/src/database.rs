use crate::pages::Page;

pub fn pages() -> Vec<Page<'static>> {
    vec![
        Page {
            id: "main",
            title: "Zhdanov Dev Website".to_string(),
            content: include_str!("../pages/main.txt").to_string(),
            links: vec!["about_me", "cv", "experiments"],
        },
        Page {
            id: "about_me",
            title: "About me".to_string(),
            content: include_str!("../pages/about_me.txt").to_string(),
            links: vec!["main", "cv"],
        },
        Page {
            id: "cv",
            title: "Zhdanov's curriculum vitae".to_string(),
            content: include_str!("../pages/cv.txt").to_string(),
            links: vec![],
        },
        Page {
            id: "404",
            title: "Page not found".to_string(),
            content: include_str!("../pages/404.txt").to_string(),
            links: vec![]
        },
        Page {
            id: "experiments",
            title: "Experiments".to_string(),
            content: include_str!("../pages/experiments.txt").to_string(),
            links: vec!["wire-world", "main"]
        },
        Page {
            id: "wire-world",
            title: "Wire-World".to_string(),
            content: "Not implemented".to_owned(),
            links: vec!["experiments"]
        },
    ]
}
