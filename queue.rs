struct Queue<T> {
  queue: Vec<T>
}
impl<T> Queue<T>{
  fn new()-> Self{
    Queue{queue: vec![]}
  }
  fn new_with_capacity(max_size: usize)-> Self{
    Self {
      queue: Vec::with_capacity(max_size)
    }
  }
  fn enqueue(&mut self, item: T) {
    self.queue.push(item)
  }
  fn dequeue(&mut self) -> T {
      self.queue.remove(0)
  }
  fn get_size(&mut self) -> usize{
    self.queue.len()
  }
  fn get_peak(&mut self) -> Option<&T>{
    self.queue.first()
  }
}

fn main(){
  let mut st:Queue<i8> = Queue{queue: vec![]};
  st.enqueue(1);
  st.enqueue(3);
  let mut st2:Queue<i8> = Queue::new_with_capacity(5);
  st2.enqueue(5);
  st2.enqueue(7);
  println!("last: {:?}", st.get_peak());
  println!("size: {}", st.get_size());

  let rm = st.dequeue();
  println!("removed: {}", rm);
}



