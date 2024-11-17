#[derive(Clone, Debug)]

pub struct Train {
    pub name: String,
    pub start: String,
    pub current_location: String,
    pub capacity: i32,
    pub total_distance: i32,
    pub packages_to_pick_up: Vec<String>,
    pub packages_picked_up: Vec<String>,
    pub packages_delivered: Vec<String>,
}

impl Train {
    pub fn new(name: String, capacity: i32, start: String) -> Train {
        Train {
            name: name.clone(),
            start: start.clone(),
            current_location: start.clone(),
            capacity,
            total_distance: 0,
            packages_delivered: [].to_vec(),
            packages_to_pick_up: [].to_vec(),
            packages_picked_up: [].to_vec(),
        }
    }
}
