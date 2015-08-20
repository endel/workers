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
  tasks: Vec<Task>
}

impl Worker {

  pub fn new(n : usize) -> Worker {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let prefix = "workers".to_string();
    Worker {
      prefix: prefix,
      conn: client.get_connection().unwrap()
    }
  }

  pub fn register(&self, task:Task) {
  }

  pub fn work(&self) {
    println!("Working...");
    let next = self.next_task();

    if next.is_ok() {
      let data = next.unwrap();
      println!("Ok: {}", data);
    } else {
      println!("Nothing more!");
    }
  }

  fn next_task(&self) -> redis::RedisResult<String> {
    return self.conn.lpop(format!("{}:tasks", self.prefix));
  }
}

#[test]
fn it_works() {
}
