use bevy::prelude::*;
use std::fmt::Debug;

fn hello() {
    println!("hello");
}

// task1: two systems
// 1. add_people
// 2. greet people
//
// task2: move two systems into one plugin

#[derive(Component, Debug)]
struct Name(String);

#[derive(Component)]
struct Person;

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Neil".to_string())));
    commands.spawn((Person, Name("Liv".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)
            // .add_system(hello)
            // .add_system(greet_people);
            .add_system(greet_people_in_period);
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people_in_period(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_system(greet_people_in_period)
        .add_plugin(HelloPlugin)
        // .add_startup_system(add_people)
        // .add_system(hello)
        // .add_system(greet_people)
        .run();
}
