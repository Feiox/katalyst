use katalyst;
use log::Level;

fn main() {
    simple_logger::init_with_level(Level::Debug).unwrap();
    katalyst::katalyst("config.yml");
}
