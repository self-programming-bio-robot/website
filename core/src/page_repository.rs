use std::{collections::HashMap};

use crate::{pages::Page, database::pages};

pub struct PageLocalRepository<'repo> {
   pages: HashMap<&'repo str, Page<'repo>>, 
}

pub trait PageRepository<'life> {
   fn get_page(&'life self, id: &str) -> Option<&'life Page>;
}

impl<'life> Default for PageLocalRepository<'life> {
    fn default() -> Self {
        Self {
            pages: pages().into_iter()
                .map(|page| (page.id.clone(), page))
                .collect(), 
        }        
    }    
}

impl<'life> PageRepository<'life> for PageLocalRepository<'life> {
    fn get_page(&'life self, id: &str) -> Option<&'life Page> {
        self.pages.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_create_default_page_local_repository() {
        let repo = PageLocalRepository::default();
        assert_eq!(repo.pages.len(), 4);
    }
    
    #[test]
    fn success_get_page_by_id() {
        let repo = PageLocalRepository::default();
        let page = repo.get_page("main");
        assert_eq!(page.is_some(), true);
    }
    
    #[test]
    fn fail_get_non_exists_page() {
        let repo = PageLocalRepository::default();
        let page = repo.get_page("non_exists");
        assert_eq!(page.is_none(), true);
    }
    
    #[test]
    fn correct_build_page() {
        let repo = PageLocalRepository::default();
        if let Some(page) = repo.get_page("main") {
            assert_eq!(page.id, "main");
            assert_eq!(page.title, "Zhdanov Dev Website".to_string());
            assert!(page.content.len() > 0);
            assert_eq!(page.links.len(), 2);
        }
    }
}
