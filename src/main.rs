mod character;
mod combat;
mod game;
mod item;
mod ui;
mod world;

fn main() {
    println!("Welcome to Echoes of the Forgotten Realm!");
    game::run();
}
