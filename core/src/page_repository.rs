use std::{collections::HashMap};

use crate::pages::{Page, Link};

pub struct PageLocalRepository {
   pages: HashMap<String, Page>, 
}

pub trait PageRepository<'life> {
   fn get_page(&'life self, id: &String) -> Option<&'life Page>;
}

impl Default for PageLocalRepository {
    fn default() -> Self {
        let pages = vec![
            Page {
                id: "main".to_string(),
                title: "Zhdanov Dev Website".to_string(),
                content: include_str!("../pages/main.md").to_string(),
                links: vec![
                    Link {
                        text: "GitHub".to_string(),
                        link: "https://github.com/KaskaRUS".to_string(),
                    }
                ]
            }
        ];
        Self { 
            pages: pages.into_iter()
                .map(|page| (page.id.clone(), page))
                .collect(), 
        }        
    }    
}

impl<'life> PageRepository<'life> for PageLocalRepository {
    fn get_page(&'life self, id: &String) -> Option<&'life Page> {
        self.pages.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_create_default_page_local_repository() {
        let repo = PageLocalRepository::default();
        assert_eq!(repo.pages.len(), 1);
    }
    
    #[test]
    fn success_get_page_by_id() {
        let repo = PageLocalRepository::default();
        let page = repo.get_page(&"main".to_string());
        assert_eq!(page.is_some(), true);
    }
    
    #[test]
    fn fail_get_non_exists_page() {
        let repo = PageLocalRepository::default();
        let page = repo.get_page(&"non_exists".to_string());
        assert_eq!(page.is_none(), true);
    }
    
    #[test]
    fn correct_build_page() {
        let repo = PageLocalRepository::default();
        if let Some(page) = repo.get_page(&"main".to_string()) {
            assert_eq!(page.id, "main".to_string());
            assert_eq!(page.title, "Zhdanov Dev Website".to_string());
            assert!(page.content.len() > 0);
            assert_eq!(page.links.len(), 1);
        }
    }
}
