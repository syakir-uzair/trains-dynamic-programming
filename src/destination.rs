use crate::route::Route;

#[derive(Clone, Debug, PartialEq)]
pub struct Destination {
    pub from: String,
    pub to: String,
    pub distance: i32,
    pub cumulative_distance: i32,
    pub checkpoints: Vec<Route>,
}

impl Destination {
    pub fn new() -> Destination {
        Destination {
            from: "".to_string(),
            to: "".to_string(),
            distance: 0,
            cumulative_distance: 0,
            checkpoints: [].to_vec(),
        }
    }
}
