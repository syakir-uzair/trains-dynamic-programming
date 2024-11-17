use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    i32,
};

use crate::{
    destination::Destination, graph::Graph, input::Input, movement::Movement, package::Package,
    train::Train,
};

pub struct Navigation {
    pub graph: Graph,
    pub trains: HashMap<String, Train>,
    pub packages: HashMap<String, Package>,
    pub cache: HashMap<String, Vec<Movement>>,
}

impl Navigation {
    pub fn new(input: Input) -> Navigation {
        let mut graph = Graph::new();
        let mut trains: HashMap<String, Train> = HashMap::new();
        let mut packages: HashMap<String, Package> = HashMap::new();

        for (_name, from, to, distance) in input.edges {
            graph.add_edge(from, to, distance);
        }

        for (name, capacity, start) in input.trains {
            trains.insert(name.clone(), Train::new(name.clone(), capacity, start));
        }

        for (name, weight, from, to) in input.packages {
            packages.insert(name.clone(), Package::new(name, weight, from, to));
        }

        Navigation {
            graph,
            trains,
            packages,
            cache: HashMap::new(),
        }
    }

    pub fn get_cache_key(
        trains: HashMap<String, Train>,
        packages: HashMap<String, Package>,
    ) -> String {
        let mut train_locations: Vec<(String, String)> = [].to_vec();
        for (train_name, train) in trains {
            train_locations.push((train_name, train.current_location));
        }
        train_locations.sort_by(|(train_a, _), (train_b, _)| train_a.cmp(train_b));

        let mut packages_to_be_picked_up: Vec<(String, String)> = [].to_vec();
        let mut packages_picked_up: Vec<(String, String)> = [].to_vec();
        let mut packages_delivered: Vec<(String, String)> = [].to_vec();

        for (package_name, pack) in packages {
            if pack.delivered_by != "" {
                packages_delivered.push((package_name.clone(), pack.delivered_by.clone()));
            }
            if pack.picked_up_by != "" {
                packages_picked_up.push((package_name.clone(), pack.picked_up_by.clone()));
            }
            if pack.to_be_picked_up_by != "" {
                packages_to_be_picked_up
                    .push((package_name.clone(), pack.to_be_picked_up_by.clone()));
            }
        }
        packages_to_be_picked_up.sort_by(|(package_a, _), (package_b, _)| package_a.cmp(package_b));
        packages_picked_up.sort_by(|(package_a, _), (package_b, _)| package_a.cmp(package_b));
        packages_delivered.sort_by(|(package_a, _), (package_b, _)| package_a.cmp(package_b));

        let train_locations_cache_key = train_locations
            .into_iter()
            .fold("train_locations".to_string(), |acc, (train, location)| {
                format!("{},{}:{}", acc, train, location)
            });
        let packages_to_be_picked_up_cache_key = packages_to_be_picked_up.into_iter().fold(
            "packages_to_be_picked_up".to_string(),
            |acc, (package, train)| format!("{},{}:{}", acc, package, train),
        );
        let packages_picked_up_cache_key = packages_picked_up
            .into_iter()
            .fold("packages_picked_up".to_string(), |acc, (package, train)| {
                format!("{},{}:{}", acc, package, train)
            });
        let packages_delivered_cache_key = packages_delivered.into_iter().fold(
            "packages_delivered".to_string(),
            |acc, (package, train)| {
                if acc == "" {
                    return format!("{}:{}", package, train);
                }
                format!("{},{}:{}", acc, package, train)
            },
        );
        format!(
            "{};{};{};{}",
            train_locations_cache_key,
            packages_to_be_picked_up_cache_key,
            packages_picked_up_cache_key,
            packages_delivered_cache_key
        )
    }
    pub fn get_capable_trains(
        pack: Package,
        packages: HashMap<String, Package>,
        trains: HashMap<String, Train>,
    ) -> Vec<Train> {
        let mut capable_trains: Vec<Train> = [].to_vec();
        for (_, train) in trains {
            let mut train_packages = train.packages_to_pick_up.clone();
            for package_picked_up in train.packages_picked_up.clone() {
                train_packages.push(package_picked_up);
            }
            let packages_total_weight: i32 = train_packages.into_iter().fold(0, |acc, package| {
                let weight = match packages.get(&package) {
                    Some(item) => item.weight,
                    None => {
                        panic!("Package not found.");
                    }
                };
                acc + weight
            });
            let train_capacity_left = train.capacity - packages_total_weight;

            if train_capacity_left >= pack.weight {
                capable_trains.push(train.clone());
            }
        }

        capable_trains
    }
    pub fn get_packages_to_deliver(
        train: Train,
        packages: HashMap<String, Package>,
        to: String,
    ) -> Vec<String> {
        let mut packages_to_deliver: Vec<String> = [].to_vec();

        for package_name in train.packages_to_pick_up {
            match packages.get(&package_name) {
                Some(package) => {
                    if package.to == to {
                        packages_to_deliver.push(package_name.clone());
                    }
                }
                None => {
                    panic!("package not found");
                }
            }
        }

        for package_name in train.packages_picked_up {
            match packages.get(&package_name) {
                Some(package) => {
                    if package.to == to {
                        packages_to_deliver.push(package_name);
                    }
                }
                None => {
                    panic!("package not found");
                }
            }
        }
        packages_to_deliver
    }

    pub fn move_train(
        train: Train,
        destination: Destination,
        packages: HashMap<String, Package>,
        movements: Vec<Movement>,
    ) -> (Vec<Movement>, Vec<String>, Vec<String>) {
        let checkpoints = destination.checkpoints;
        let mut new_movements = movements.clone();
        let mut train_movements: Vec<Movement> = [].to_vec();
        for movement in new_movements.clone() {
            if movement.train == train.name {
                train_movements.push(movement);
            }
        }

        let mut start_time = 0;
        if train_movements.len() > 0 {
            start_time = train_movements[train_movements.len() - 1].end_time;
        }

        let mut end_time = start_time + destination.distance;
        if checkpoints.len() > 0 {
            end_time = start_time + checkpoints[0].distance;
        }

        let mut packages_picked_up: Vec<String> = vec![];
        let mut packages_delivered: Vec<String> = vec![];
        if destination.cumulative_distance == 0 {
            return (
                new_movements.clone(),
                packages_picked_up.clone(),
                packages_delivered.clone(),
            );
        }

        for package in train.packages_to_pick_up.clone() {
            packages_picked_up.push(package);
        }

        let mut to = destination.to.clone();
        if checkpoints.len() > 0 {
            to = checkpoints[0].to.clone();
        }

        let packages_to_deliver =
            Navigation::get_packages_to_deliver(train.clone(), packages.clone(), to.clone());

        new_movements.push(Movement {
            start_time,
            end_time,
            from: destination.from.clone(),
            to: to.clone(),
            train: train.name.clone(),
            packages_picked_up: packages_picked_up.clone(),
            packages_delivered: packages_to_deliver.clone(),
        });

        for package in packages_to_deliver.clone() {
            packages_delivered.push(package);
        }

        let mut i = 0;
        let checkpoints_len = checkpoints.len();
        for checkpoint in checkpoints.clone() {
            let start_time = end_time;
            let mut end_time = start_time + destination.distance;
            if i < checkpoints_len - 1 {
                end_time = start_time + checkpoints[i + 1].distance;
            }

            let mut to = destination.to.clone();
            if i < checkpoints_len - 1 {
                to = checkpoints[i + 1].to.clone();
            }

            let packages_to_deliver =
                Navigation::get_packages_to_deliver(train.clone(), packages.clone(), to.clone());

            new_movements.push(Movement {
                start_time,
                end_time,
                train: train.name.clone(),
                from: checkpoint.to.clone(),
                to: to.clone(),
                packages_picked_up: vec![],
                packages_delivered: packages_to_deliver.clone(),
            });
            for package in packages_to_deliver.clone() {
                packages_delivered.push(package);
            }
            i += 1;
        }

        return (
            new_movements.clone(),
            packages_picked_up.clone(),
            packages_delivered.clone(),
        );
    }
    pub fn get_longest_distance_in_movements(movements: Vec<Movement>) -> i32 {
        let mut longest_distance = 0;
        for movement in movements.clone() {
            if movement.end_time > longest_distance {
                longest_distance = movement.end_time
            }
        }
        longest_distance
    }
    pub fn get_number_of_trains(movements: Vec<Movement>) -> i32 {
        let mut trains: HashSet<String> = HashSet::new();
        for movement in movements.clone() {
            trains.insert(movement.train.clone());
        }
        trains.len() as i32
    }
    pub fn calculate(
        &mut self,
        trains: HashMap<String, Train>,
        packages: HashMap<String, Package>,
        movements: Vec<Movement>,
    ) -> Vec<Movement> {
        let mut min_distance: i32 = i32::MAX;
        let mut min_trains: i32 = i32::MAX;
        let mut total_combinations: i32 = 0;
        let cache_key = Navigation::get_cache_key(trains.clone(), packages.clone());
        let mut best_movements: Vec<Movement> = movements.clone();
        let mut cache_found = false;
        match self.cache.get(&cache_key) {
            Some(cached_movements) => {
                best_movements = cached_movements.clone();
                cache_found = true;
            }
            None => {}
        };

        if cache_found {
            return best_movements;
        }

        let mut queue: Vec<(String, String, bool)> = vec![];

        for (_, package) in packages.clone() {
            if package.to_be_picked_up_by != ""
                || package.picked_up_by != ""
                || package.delivered_by != ""
            {
                continue;
            }

            let capable_trains =
                Navigation::get_capable_trains(package.clone(), packages.clone(), trains.clone());
            for train in capable_trains.clone() {
                queue.push((train.name.clone(), package.name.clone(), true));
            }
        }

        for (_, train) in trains.clone() {
            for package_name in train.packages_to_pick_up {
                queue.push((train.name.clone(), package_name.clone(), false));
            }
            for package_name in train.packages_picked_up {
                queue.push((train.name.clone(), package_name.clone(), false));
            }
        }

        queue.sort_by(
            |(train_name_a, package_name_a, to_pick_up_a),
             (train_name_b, package_name_b, to_pick_up_b)| {
                let train_name_cmp = train_name_a.cmp(train_name_b);
                let package_name_cmp = package_name_a.cmp(package_name_b);
                let to_pick_up_cmp = to_pick_up_a.cmp(to_pick_up_b);
                if to_pick_up_cmp == Ordering::Equal {
                    if train_name_cmp == Ordering::Equal {
                        return package_name_cmp;
                    }
                    return train_name_cmp;
                }
                return to_pick_up_cmp;
            },
        );

        for (train_name, package_name, to_pick_up) in queue {
            let mut new_trains = trains.clone();
            let mut new_packages = packages.clone();
            let mut new_train = match new_trains.get(&train_name) {
                Some(train) => train.clone(),
                None => panic!("Train not found"),
            };
            let mut new_package = match new_packages.get(&package_name) {
                Some(package) => package.clone(),
                None => panic!("Package not found"),
            };
            let mut destination = self
                .graph
                .get_destination(new_train.current_location.clone(), new_package.to.clone());
            if to_pick_up {
                destination = self
                    .graph
                    .get_destination(new_train.current_location.clone(), new_package.from.clone());
            }

            let (new_movements, packages_picked_up, packages_delivered) = Navigation::move_train(
                new_train.clone(),
                destination.clone(),
                new_packages.clone(),
                movements.clone(),
            );

            if destination.cumulative_distance > 0 {
                new_train.current_location = destination.to;
                new_train.total_distance += destination.cumulative_distance;
            }

            if to_pick_up {
                new_train.packages_to_pick_up.push(new_package.name.clone());
                new_package.to_be_picked_up_by = new_train.name.clone();
            }

            for package_name in packages_picked_up {
                let mut train_new_packages_to_pick_up: Vec<String> = vec![];
                for package_to_pick_up in new_train.packages_to_pick_up.clone() {
                    if package_to_pick_up != package_name {
                        train_new_packages_to_pick_up.push(package_to_pick_up);
                    }
                }
                new_train.packages_to_pick_up = train_new_packages_to_pick_up.clone();
                // new_train.packages_to_pick_up = vec![];
                new_train.packages_picked_up.push(package_name.clone());
                if new_package.name == package_name {
                    // new_package.to_be_picked_up_by = "".to_string();
                    new_package.picked_up_by = new_train.name.clone();
                } else {
                    let package = match new_packages.get(&package_name) {
                        Some(package) => package.clone(),
                        None => panic!("Package not found"),
                    };

                    new_packages.insert(
                        package_name.clone(),
                        Package {
                            name: package_name.clone(),
                            from: package.from.clone(),
                            to: package.to.clone(),
                            weight: package.weight,
                            to_be_picked_up_by: new_train.name.clone(),
                            picked_up_by: new_train.name.clone(),
                            delivered_by: "".to_string(),
                        },
                    );
                }
            }

            for package_name in packages_delivered {
                let mut train_new_packages_picked_up: Vec<String> = vec![];
                for package_picked_up in new_train.packages_picked_up.clone() {
                    if package_picked_up != package_name {
                        train_new_packages_picked_up.push(package_picked_up);
                    }
                }
                new_train.packages_picked_up = train_new_packages_picked_up.clone();
                new_train.packages_delivered.push(package_name.clone());
                if new_package.name == package_name {
                    // new_package.picked_up_by = "".to_string();
                    new_package.delivered_by = new_train.name.clone();
                } else {
                    let package = match new_packages.get(&package_name) {
                        Some(package) => package.clone(),
                        None => panic!("Package not found"),
                    };

                    new_packages.insert(
                        package_name.clone(),
                        Package {
                            name: package_name.clone(),
                            from: package.from.clone(),
                            to: package.to.clone(),
                            weight: package.weight,
                            to_be_picked_up_by: new_train.name.clone(),
                            picked_up_by: new_train.name.clone(),
                            delivered_by: new_train.name.clone(),
                        },
                    );
                }
            }

            new_trains.insert(new_train.name.clone(), new_train.clone());
            new_packages.insert(new_package.name.clone(), new_package.clone());

            let all_movements = self.calculate(
                new_trains.clone(),
                new_packages.clone(),
                new_movements.clone(),
            );
            let longest_train_distance =
                Navigation::get_longest_distance_in_movements(all_movements.clone());
            let number_of_trains = Navigation::get_number_of_trains(all_movements.clone());

            if longest_train_distance < min_distance
                || (longest_train_distance == min_distance && number_of_trains < min_trains)
            {
                min_distance = longest_train_distance;
                min_trains = number_of_trains;
                best_movements = all_movements.clone();
            }
            total_combinations += 1;
        }

        // If no combinations left, it could mean either:
        if total_combinations == 0 {
            let mut undelivered_packages: Vec<String> = vec![];
            for (_, package) in packages {
                if package.delivered_by == "" {
                    undelivered_packages.push(package.name.clone());
                }
            }

            // All packages are delivered
            if undelivered_packages.len() == 0 {
                return best_movements;
            // Or something is wrong
            } else {
                panic!("No solution found");
            }
        }

        best_movements.sort_by(|movement_a, movement_b| {
            let train_cmp = movement_a.train.cmp(&movement_b.train);
            let start_time_cmp = movement_a.start_time.cmp(&movement_b.start_time);
            if train_cmp == Ordering::Equal {
                return start_time_cmp;
            }
            return train_cmp;
        });
        self.cache.insert(cache_key.clone(), best_movements.clone());
        return best_movements;
    }
}

#[cfg(test)]
mod tests {
    use crate::{input::Input, movement::Movement};

    use super::Navigation;

    #[test]
    fn test_basic_navigation() {
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
        assert_eq!(
            movements,
            vec![
                Movement {
                    start_time: 0,
                    end_time: 30,
                    from: "B".to_string(),
                    to: "A".to_string(),
                    train: "Q1".to_string(),
                    packages_picked_up: vec![],
                    packages_delivered: vec![],
                },
                Movement {
                    start_time: 30,
                    end_time: 60,
                    from: "A".to_string(),
                    to: "B".to_string(),
                    train: "Q1".to_string(),
                    packages_picked_up: vec!["K1".to_string()],
                    packages_delivered: vec![],
                },
                Movement {
                    start_time: 60,
                    end_time: 70,
                    from: "B".to_string(),
                    to: "C".to_string(),
                    train: "Q1".to_string(),
                    packages_picked_up: vec![],
                    packages_delivered: vec!["K1".to_string()],
                }
            ]
        );
    }
    #[test]
    fn test_delivering_package_using_multiple_trains_in_parallel() {
        let mut navigation = Navigation::new(Input {
            edges: vec![
                ("E1".to_string(), "A".to_string(), "X".to_string(), 10),
                ("E2".to_string(), "B".to_string(), "X".to_string(), 10),
                ("E3".to_string(), "C".to_string(), "X".to_string(), 10),
                ("E4".to_string(), "D".to_string(), "X".to_string(), 10),
                ("E5".to_string(), "E".to_string(), "X".to_string(), 10),
                ("E6".to_string(), "F".to_string(), "X".to_string(), 10),
            ],
            packages: vec![
                ("K1".to_string(), 5, "X".to_string(), "D".to_string()),
                ("K2".to_string(), 5, "X".to_string(), "E".to_string()),
                ("K3".to_string(), 5, "X".to_string(), "F".to_string()),
            ],
            trains: vec![
                ("Q1".to_string(), 15, "A".to_string()),
                ("Q2".to_string(), 15, "B".to_string()),
                ("Q3".to_string(), 15, "C".to_string()),
            ],
        });
        let movements = navigation.calculate(
            navigation.trains.clone(),
            navigation.packages.clone(),
            vec![],
        );
        assert_eq!(
            movements,
            vec![
                Movement {
                    start_time: 0,
                    end_time: 10,
                    from: "A".to_string(),
                    to: "X".to_string(),
                    train: "Q1".to_string(),
                    packages_picked_up: [].to_vec(),
                    packages_delivered: [].to_vec(),
                },
                Movement {
                    start_time: 10,
                    end_time: 20,
                    from: "X".to_string(),
                    to: "D".to_string(),
                    train: "Q1".to_string(),
                    packages_picked_up: ["K1".to_string()].to_vec(),
                    packages_delivered: ["K1".to_string()].to_vec(),
                },
                Movement {
                    start_time: 0,
                    end_time: 10,
                    from: "B".to_string(),
                    to: "X".to_string(),
                    train: "Q2".to_string(),
                    packages_picked_up: [].to_vec(),
                    packages_delivered: [].to_vec(),
                },
                Movement {
                    start_time: 10,
                    end_time: 20,
                    from: "X".to_string(),
                    to: "E".to_string(),
                    train: "Q2".to_string(),
                    packages_picked_up: ["K2".to_string()].to_vec(),
                    packages_delivered: ["K2".to_string()].to_vec(),
                },
                Movement {
                    start_time: 0,
                    end_time: 10,
                    from: "C".to_string(),
                    to: "X".to_string(),
                    train: "Q3".to_string(),
                    packages_picked_up: [].to_vec(),
                    packages_delivered: [].to_vec(),
                },
                Movement {
                    start_time: 10,
                    end_time: 20,
                    from: "X".to_string(),
                    to: "F".to_string(),
                    train: "Q3".to_string(),
                    packages_picked_up: ["K3".to_string()].to_vec(),
                    packages_delivered: ["K3".to_string()].to_vec(),
                },
            ],
        );
    }
    // #[test]
    // fn test_delivering_more_packages_using_multiple_trains_in_parallel() {
    //     let mut navigation = Navigation::new(Input {
    //         edges: vec![
    //             ("E1".to_string(), "A".to_string(), "B".to_string(), 30),
    //             ("E2".to_string(), "B".to_string(), "G".to_string(), 30),
    //             ("E3".to_string(), "G".to_string(), "H".to_string(), 30),
    //             ("E4".to_string(), "H".to_string(), "B".to_string(), 30),
    //             ("E5".to_string(), "B".to_string(), "C".to_string(), 30),
    //             ("E6".to_string(), "C".to_string(), "D".to_string(), 30),
    //             ("E7".to_string(), "D".to_string(), "E".to_string(), 30),
    //             ("E8".to_string(), "C".to_string(), "F".to_string(), 30),
    //             ("E9".to_string(), "F".to_string(), "E".to_string(), 30),
    //         ],
    //         packages: vec![
    //             ("K1".to_string(), 5, "A".to_string(), "G".to_string()),
    //             ("K2".to_string(), 5, "A".to_string(), "H".to_string()),
    //             ("K3".to_string(), 5, "B".to_string(), "H".to_string()),
    //             ("K4".to_string(), 5, "H".to_string(), "E".to_string()),
    //             ("K5".to_string(), 5, "E".to_string(), "A".to_string()),
    //             ("K6".to_string(), 5, "F".to_string(), "C".to_string()),
    //             ("K7".to_string(), 5, "F".to_string(), "G".to_string()),
    //         ],
    //         trains: vec![
    //             ("Q1".to_string(), 20, "B".to_string()),
    //             ("Q2".to_string(), 20, "C".to_string()),
    //         ],
    //     });
    //     let movements = navigation.calculate(
    //         navigation.trains.clone(),
    //         navigation.packages.clone(),
    //         vec![],
    //     );
    //     assert_eq!(movements, vec![]);
    // }
}
