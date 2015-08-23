extern crate redis;
use redis::{Connection, Commands};

extern crate threadpool;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

pub trait Task {
  fn get_name(&self) -> &str;
  fn perform(&self) -> bool;
}

pub struct Worker {
  prefix: String,
  conn: redis::Connection,
  tasks: Vec<Box<Task>>
}

impl Worker {

  pub fn new(n : usize) -> Worker {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let prefix = "workers".to_string();
    Worker {
      prefix: prefix,
      conn: client.get_connection().unwrap(),
      tasks: Vec::new()
    }
  }

  pub fn register(&mut self, t: Box<Task>) {
    self.tasks.push(t);
  }

  pub fn work(&self) {
    println!("Working...");
    let next = self.next_task();

    if next.is_ok() {
      let data = next.unwrap();
      data.perform();
    } else {
      println!("Nothing more!");
    }
  }

  fn next_task(&self) -> Result<&Box<Task>, &'static str> {
    let task_name : redis::RedisResult<String> = self.conn.lpop(format!("{}:tasks", self.prefix));

    // TODO: it should be a better way to write this
    if task_name.is_ok() {
      let r = self.get_task_by_name(task_name.unwrap());
      if r.is_ok() {
        Ok(r.unwrap())
      } else {
        r
      }

    } else {
      Err("No more")
    }
  }

  fn get_task_by_name(&self, name: String) -> Result<&Box<Task>, &'static str> {
    for task in &self.tasks {
      if task.get_name() == name {
        return Ok(task)
      }
    }
    Err("Task not found!")
  }

}

#[test]
fn it_works() {
}
