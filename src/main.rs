mod backend;
use backend::*;

mod cli;
use cli::*;

fn main() {
    let mut backend = SectionManager::load_from_file();

    cli_manager::start(&mut backend);

    backend.save_to_file();
}
