use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_startup_system(add_people)
        .add_system(hello_world)
        .run();
}

fn hello_world() {
    println!("Hello world!");
}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Elaina Proctor".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Renzo Hume".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Zayna Nieves".to_string()));
}
