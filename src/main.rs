use std::time::Instant;

use crate::input::Input;
use crate::navigation::Navigation;

pub mod destination;
pub mod graph;
pub mod input;
pub mod min_heap;
pub mod movement;
pub mod navigation;
pub mod package;
pub mod route;
pub mod train;

fn main() {
    let start = Instant::now();
    let mut navigation = Navigation::new(Input {
        edges: vec![
            ("E1".to_string(), "A".to_string(), "B".to_string(), 30),
            ("E2".to_string(), "B".to_string(), "C".to_string(), 10),
        ],
        packages: vec![("K1".to_string(), 5, "A".to_string(), "C".to_string())],
        trains: vec![("Q1".to_string(), 6, "B".to_string())],
    });
    let movements = navigation.calculate(
        navigation.trains.clone(),
        navigation.packages.clone(),
        vec![],
    );
    let duration = start.elapsed();

    println!("Movements: {:#?}", movements);
    println!("Duration: {:?}", duration);
    println!("Run `cargo t` to test all cases");
}
