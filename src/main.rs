use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new().add_system(hello_world).run();
}

fn hello_world() {
    println!("Hello world!");
}
