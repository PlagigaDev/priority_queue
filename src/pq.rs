struct PriorityObject<T>{
	priority: u32,
	value: T
}

pub struct PriorityQueue<T> {
	list: Vec<PriorityObject<T>>
}

impl<T> PriorityQueue<T> {
	pub fn new() -> Self{
		PriorityQueue { list: Vec::new() }
	}
	pub fn add(&mut self, value: T, priority: u32){
		//we need to know the position of where we need to insert the object, this goes from 0 (lowest priority) upwards
		let index_to_insert = self.list.iter().position(|obj| obj.priority < priority);
		match index_to_insert {
			Some(index) => self.list.insert(index, PriorityObject { priority: priority, value: value }),
			None => self.list.push(PriorityObject { priority: priority, value: value })
		}
	}
	pub fn remove(&mut self, priority: u32) -> Option<T>{
		match self.list.iter().position(|obj| obj.priority == priority) {
			Some(index) => Some(self.list.remove(index).value),
			None => None
			
		}
	}
	pub fn get(&self, priority: u32) -> Option<&T>{
		match self.list.iter().find(|obj| obj.priority == priority) {
			Some(obj) => Some(&obj.value),
			None => None
		}
	}
	pub fn len(&self) -> usize {
		self.list.len()
	}
	pub fn is_empty(&self) -> bool {
		self.list.is_empty()
	}
}