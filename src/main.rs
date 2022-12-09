mod backend;
use backend::*;

fn main() {
    let mut backend = SectionManager::load_from_file();

    println!("{}", backend.add_section("Todoing2"));
    println!("{}", backend.select_section("Todoing2"));

    println!("{:?}", backend);

    backend.save_to_file();
}
