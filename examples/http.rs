use std::thread;

extern crate redis;
use redis::Commands;

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

struct SomeTask;
impl Task for SomeTask {
  fn get_name(&self) -> &str { "some_task" }
  fn perform(&self, params: Result<String, ()>) -> bool {
    if params.is_ok() {
      let data : Vec<i32> = json::decode(&params.unwrap()).unwrap();
      println!("Performing SomeTask task: {}", data.len());

    } else {
      println!("Invalid arguments for this job");
    }
    true
  }
}

fn add_dummy_tasks(worker: &Worker, task_name: &str) {
  // add dummy tasks in "some_task" queue
  for i in 1..3 {
    worker.enqueue(task_name, "[]");
    worker.enqueue(task_name, "[1]");
    worker.enqueue(task_name, "[1,2,3]");
  }
}

fn main() {
  let mut worker = Worker::new(4);

  add_dummy_tasks(&worker, "http_worker");
  add_dummy_tasks(&worker, "some_task");

  worker.register( Box::new(HttpWorker) );
  worker.register( Box::new(SomeTask) );

  loop {
    worker.work();
    thread::sleep_ms(1000);
  }
}
