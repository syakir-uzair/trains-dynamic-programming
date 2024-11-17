#[derive(Clone, Debug)]
pub struct Package {
    pub name: String,
    pub from: String,
    pub to: String,
    pub weight: i32,
    pub to_be_picked_up_by: String,
    pub picked_up_by: String,
    pub delivered_by: String,
}

impl Package {
    pub fn new(name: String, weight: i32, from: String, to: String) -> Package {
        Package {
            name: name.clone(),
            from: from.clone(),
            to: to.clone(),
            weight,
            to_be_picked_up_by: "".to_string(),
            picked_up_by: "".to_string(),
            delivered_by: "".to_string(),
        }
    }
}
