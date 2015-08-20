extern crate workers;

use std::thread;
use workers::{Task, Worker};

struct HttpWorker;
impl Task for HttpWorker {
  fn get_name(&self) -> &str {
    "http_worker"
  }
  fn perform(&self) -> bool {
    true
  }
}

fn main() {
  let worker = Worker::new(4);
  worker.register(HttpWorker);

  loop {
    worker.work();
    let w = HttpWorker;
    println!("What's the name? {}", w.get_name());
    thread::sleep_ms(1000);
  }
}
