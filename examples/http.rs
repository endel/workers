extern crate workers;

use std::thread;
use workers::{Task, Worker};

struct HttpWorker;
impl Task for HttpWorker {
  fn get_name(&self) -> &str {
    "http_worker"
  }
  fn perform(&self) -> bool {
    println!("Performing HttpWorker task...");
    true
  }
}

fn main() {
  let mut worker = Worker::new(4);
  worker.register( Box::new(HttpWorker) );

  loop {
    worker.work();
    thread::sleep_ms(1000);
  }
}
