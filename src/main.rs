mod backend;
use backend::*;

fn main() {
    let mut backend = SectionManager::load_from_file();

    backend.save_to_file();
}
