use std::io::{stdout, Write};

use crossterm::{Result, cursor, terminal, style::Print, queue};
use zhdanov_website_core::{pages::Page, page_repository::PageRepository};

pub struct PageController<'page, 'repo> {
    current_page: &'page Page<'page>,
    repository: &'repo dyn PageRepository<'page>,
    main_page: &'repo str,
    not_found_page: &'repo str,
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input;
}

impl<'life> PageController<'life, 'life> {
    
    pub fn new(
        page_repository: &'life dyn PageRepository<'life>, 
        start_page: &str
    ) -> Self {
        let page = page_repository.get_page(&start_page.to_owned())
            .expect("Started page not found");
        
        Self {
            current_page: page, 
            repository: page_repository, 
            main_page: "main",
            not_found_page: "404",
        }
    }

    pub fn print_current_page(&self) -> Result<()> {
        let mut stdout = stdout();
        queue!(stdout,
               terminal::Clear(terminal::ClearType::All),
               cursor::MoveTo(0, 0),
               Print(&self.current_page.content),
        )?;
        stdout.flush()?;

        Ok(())
    }

    pub fn wait_input(&mut self) -> Result<()> {
        let range = 0..self.current_page.links.len();

        if range.len() == 0 {
            read_line();
            self.change_page(self.main_page);
            Ok(())
        } else {
            println!(""); // add empty line
            let welcome_message = 
                format!("Enter number of next page from 0 to {}", range.len()-1);
            println!("{}", &welcome_message);

            loop {
                match read_line().trim().parse::<usize>() {
                    Ok(next) if range.contains(&next) => {
                        let link: &str = match self.current_page.links.get(next) {
                            Some(link) => link,
                            None => self.not_found_page,
                        };      
                        self.change_page(link);
                        return Ok(())
                    },
                    Ok(_) | Err(_) => {
                        println!("{}", &welcome_message);
                    }
                }
            }
        }
    }

    fn change_page(&mut self, page: &str) {
        self.current_page = 
            if let Some(page) = self.repository.get_page(page) {
                page   
            } else {
                self.repository
                    .get_page(self.not_found_page).unwrap()
            }
    }
}
