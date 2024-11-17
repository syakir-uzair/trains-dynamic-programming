#[derive(Clone, PartialEq, Debug)]

pub struct Movement {
    pub start_time: i32,
    pub end_time: i32,
    pub from: String,
    pub to: String,
    pub train: String,
    pub packages_picked_up: Vec<String>,
    pub packages_delivered: Vec<String>,
}
