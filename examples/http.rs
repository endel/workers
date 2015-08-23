use std::thread;

extern crate rustc_serialize;
use rustc_serialize::json;

extern crate workers;
use workers::{Task, Worker};

struct HttpWorker;
impl Task for HttpWorker {
  fn get_name(&self) -> &str { "http_worker" }
  fn perform(&self, params: Result<String, ()>) -> bool {
    if params.is_ok() {
      let data : Vec<i32> = json::decode(&params.unwrap()).unwrap();
      println!("Performing HttpWorker task: {}", data.len());

    } else {
      println!("Invalid arguments for this job");
    }
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
