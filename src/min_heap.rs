use crate::route::Route;

pub struct MinHeap {
    pub heap: Vec<Route>,
}

impl MinHeap {
    pub fn new() -> MinHeap {
        MinHeap { heap: [].to_vec() }
    }
    pub fn get_left_child_index(parent_index: i32) -> i32 {
        2 * parent_index + 1
    }
    pub fn get_right_child_index(parent_index: i32) -> i32 {
        2 * parent_index + 2
    }
    pub fn get_parent_index(child_index: i32) -> i32 {
        let mid: f32 = (child_index as f32 - 1.0) / 2.0;
        mid.floor() as i32
    }
    pub fn has_left_child(&mut self, index: i32) -> bool {
        MinHeap::get_left_child_index(index) < self.heap.len() as i32
    }
    pub fn has_right_child(&mut self, index: i32) -> bool {
        MinHeap::get_right_child_index(index) < self.heap.len() as i32
    }
    pub fn has_parent(index: i32) -> bool {
        MinHeap::get_parent_index(index) >= 0
    }
    pub fn left_child(&mut self, index: i32) -> Route {
        self.heap[MinHeap::get_left_child_index(index) as usize].clone()
    }
    pub fn right_child(&mut self, index: i32) -> Route {
        self.heap[MinHeap::get_right_child_index(index) as usize].clone()
    }
    pub fn parent(&mut self, index: i32) -> Route {
        self.heap[MinHeap::get_parent_index(index) as usize].clone()
    }
    pub fn swap(&mut self, index_one: i32, index_two: i32) {
        let temp = self.heap[index_one as usize].clone();
        self.heap[index_one as usize] = self.heap[index_two as usize].clone();
        self.heap[index_two as usize] = temp;
    }
    pub fn peek(&mut self) -> Option<Route> {
        if self.heap.len() != 0 {
            return Some(self.heap[0].clone());
        }
        None
    }
    pub fn remove(&mut self) -> Option<Route> {
        if self.heap.len() == 0 {
            return None;
        }
        let route: Option<Route> = Some(self.heap[0].clone());
        self.heap[0] = self.heap[self.heap.len() - 1].clone();
        self.heap.pop();
        self.heapify_down();
        return route;
    }
    pub fn add(&mut self, route: Route) {
        self.heap.push(route);
        self.heapify_up();
    }
    pub fn heapify_up(&mut self) {
        let mut index: i32 = self.heap.len() as i32 - 1;
        while MinHeap::has_parent(index)
            && MinHeap::parent(self, index).distance > self.heap[index as usize].distance
        {
            MinHeap::swap(self, MinHeap::get_parent_index(index), index);
            index = MinHeap::get_parent_index(index);
        }
    }
    pub fn heapify_down(&mut self) {
        let mut index = 0;
        while MinHeap::has_left_child(self, index) {
            let mut smaller_child_index = MinHeap::get_left_child_index(index);
            if MinHeap::has_right_child(self, index)
                && MinHeap::right_child(self, index).distance
                    < MinHeap::left_child(self, index).distance
            {
                smaller_child_index = MinHeap::get_right_child_index(index);
            }
            if self.heap[index as usize].distance < self.heap[smaller_child_index as usize].distance
            {
                break;
            } else {
                MinHeap::swap(self, index, smaller_child_index);
            }
            index = smaller_child_index;
        }
    }
}

pub fn create_min_heap() -> MinHeap {
    MinHeap { heap: [].to_vec() }
}

#[cfg(test)]
mod tests {
    use crate::{min_heap::MinHeap, route::Route};

    #[test]
    fn test_min_heap() {
        let mut min_heap = MinHeap::new();
        min_heap.add(Route {
            to: "a".to_string(),
            distance: 1,
        });
        min_heap.add(Route {
            to: "b".to_string(),
            distance: 5,
        });
        min_heap.add(Route {
            to: "c".to_string(),
            distance: 2,
        });
        let res1 = min_heap.remove();
        assert!(res1.is_some_and(|route| route.to == "a"));
        let res2 = min_heap.remove();
        assert!(res2.is_some_and(|route| route.to == "c"));
        min_heap.add(Route {
            to: "d".to_string(),
            distance: 3,
        });
        let res3 = min_heap.remove();
        assert!(res3.is_some_and(|route| route.to == "d"));
        let res4 = min_heap.remove();
        assert!(res4.is_some_and(|route| route.to == "b"));
        let res5 = min_heap.remove();
        assert!(res5.is_none());
    }
}
