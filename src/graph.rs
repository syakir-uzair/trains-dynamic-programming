use std::{
    collections::{HashMap, HashSet},
    i32,
};

use crate::{destination::Destination, min_heap::MinHeap, route::Route};

pub struct Graph {
    pub adj_list: HashMap<String, Vec<Route>>,
    pub cache: HashMap<String, HashMap<String, Destination>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            adj_list: HashMap::new(),
            cache: HashMap::new(),
        }
    }
    pub fn add_edge(&mut self, from: String, to: String, distance: i32) {
        self.adj_list
            .entry(from.clone())
            .and_modify(|froms| {
                froms.push(Route {
                    to: to.clone(),
                    distance: distance,
                })
            })
            .or_insert(
                [Route {
                    to: to.clone(),
                    distance: distance,
                }]
                .to_vec(),
            );
        self.adj_list
            .entry(to.clone())
            .and_modify(|tos| {
                tos.push(Route {
                    to: from.clone(),
                    distance: distance,
                })
            })
            .or_insert(
                [Route {
                    to: from.clone(),
                    distance: distance,
                }]
                .to_vec(),
            );
    }
    pub fn calculate_neighbour(
        neighbour: Route,
        start: String,
        current: String,
        current_distance: i32,
        destinations: HashMap<String, Destination>,
    ) -> Option<Destination> {
        let next = neighbour.to.clone();
        let distance = neighbour.distance;

        let new_cumulative_distance = current_distance + distance;
        let cumulative_distance: i32;
        let mut prev_destination: Destination = Destination::new();
        let mut prev_checkpoints: Vec<Route> = [].to_vec();
        match destinations.get(&next) {
            Some(destination) => {
                cumulative_distance = destination.cumulative_distance;
            }
            None => cumulative_distance = i32::MAX,
        }
        match destinations.get(&current) {
            Some(dest) => {
                prev_destination = dest.clone();
                prev_checkpoints = prev_destination.checkpoints.clone();
            }
            None => {}
        }

        if new_cumulative_distance < cumulative_distance {
            let mut checkpoints = [].to_vec();
            if prev_destination.checkpoints.len() > 0 {
                checkpoints = prev_checkpoints.clone();
                checkpoints.push(Route {
                    to: current.clone(),
                    distance: current_distance
                        - prev_checkpoints[prev_checkpoints.len() - 1].distance,
                })
            } else if current != start {
                checkpoints.push(Route {
                    to: current.clone(),
                    distance: current_distance,
                });
            }
            return Some(Destination {
                from: start.clone(),
                to: next.clone(),
                checkpoints,
                distance,
                cumulative_distance: new_cumulative_distance,
            });
        }
        None
    }
    pub fn dijkstra(&mut self, start: String) -> HashMap<String, Destination> {
        let mut destinations = match self.cache.get(&start) {
            Some(cached_desttinations) => cached_desttinations.clone(),
            None => HashMap::new(),
        };

        if destinations.len() > 0 {
            return destinations;
        }

        let mut min_heap = MinHeap::new();
        let mut visited: HashSet<String> = HashSet::new();

        for (to, _route) in self.adj_list.iter() {
            destinations.insert(
                start.clone(),
                Destination {
                    from: start.clone(),
                    to: to.clone(),
                    checkpoints: [].to_vec(),
                    distance: 0,
                    cumulative_distance: i32::MAX,
                },
            );
        }

        destinations.insert(
            start.clone(),
            Destination {
                from: start.clone(),
                to: start.clone(),
                checkpoints: [].to_vec(),
                distance: 0,
                cumulative_distance: 0,
            },
        );

        min_heap.add(Route {
            to: start.clone(),
            distance: 0,
        });

        while min_heap.heap.len() > 0 {
            let current: String;
            let current_distance: i32;
            match MinHeap::remove(&mut min_heap) {
                Some(min) => {
                    current = min.to;
                    current_distance = min.distance;
                }
                None => {
                    panic!("Minimum distance not found");
                }
            }

            if visited.contains(&current) {
                continue;
            }

            visited.insert(current.clone());

            let mut neighbours: Vec<Route> = [].to_vec();
            match self.adj_list.get(&current) {
                Some(routes) => {
                    neighbours = routes.clone();
                }
                None => {}
            }

            for neighbour in neighbours {
                let destination_option = Graph::calculate_neighbour(
                    neighbour,
                    start.clone(),
                    current.clone(),
                    current_distance,
                    destinations.clone(),
                );

                match destination_option {
                    Some(destination) => {
                        destinations.insert(destination.to.clone(), destination.clone());
                        min_heap.add(Route {
                            to: destination.to.clone(),
                            distance: destination.cumulative_distance,
                        });
                    }
                    None => {}
                }
            }
        }

        self.cache.insert(start.clone(), destinations.clone());
        return destinations;
    }
    pub fn get_destination(&mut self, from: String, to: String) -> Destination {
        match Graph::dijkstra(self, from).get(&to) {
            Some(dest) => dest.clone(),
            None => {
                panic!("Destination not found");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{destination::Destination, route::Route};

    use super::Graph;

    #[test]
    fn test_graph() {
        let mut graph = Graph::new();
        graph.add_edge("A".to_string(), "B".to_string(), 40);
        graph.add_edge("A".to_string(), "C".to_string(), 10);
        graph.add_edge("B".to_string(), "C".to_string(), 20);
        graph.add_edge("B".to_string(), "D".to_string(), 10);
        graph.add_edge("C".to_string(), "D".to_string(), 50);
        let destination = graph.get_destination("A".to_string(), "D".to_string());
        assert_eq!(
            destination,
            Destination {
                from: "A".to_string(),
                to: "D".to_string(),
                distance: 10,
                cumulative_distance: 40,
                checkpoints: [
                    Route {
                        to: "C".to_string(),
                        distance: 10,
                    },
                    Route {
                        to: "B".to_string(),
                        distance: 20,
                    }
                ]
                .to_vec()
            }
        );
    }

    #[test]
    #[should_panic]
    fn test_graph_failure() {
        let mut graph = Graph::new();
        graph.add_edge("A".to_string(), "B".to_string(), 40);
        graph.add_edge("A".to_string(), "C".to_string(), 10);
        graph.add_edge("B".to_string(), "C".to_string(), 20);
        graph.add_edge("B".to_string(), "D".to_string(), 10);
        graph.add_edge("C".to_string(), "D".to_string(), 50);
        graph.get_destination("A".to_string(), "X".to_string());
    }
}
