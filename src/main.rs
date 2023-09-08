mod pq;

use pq::PriorityQueue;

fn main() {
    let mut test: PriorityQueue<u32> = PriorityQueue::new();
    test.add(10, 1);
    println!("{}, {}", test.is_empty(), test.get(1).unwrap());
    
}
