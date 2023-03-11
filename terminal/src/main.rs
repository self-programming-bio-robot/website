mod page_controller;

use crossterm::Result;
use page_controller::PageController;
use zhdanov_website_core::page_repository::PageLocalRepository;

fn main() -> Result<()> {
    let repo = PageLocalRepository::default();
    let mut controller = PageController::new(&repo, "main");

    loop {
        controller.print_current_page()?;
        controller.wait_input()?;
    }
}
