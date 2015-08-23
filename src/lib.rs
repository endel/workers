extern crate redis;
use redis::{Connection, Commands};

extern crate threadpool;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

pub trait Task {
  fn get_name(&self) -> &str;
  fn perform(&self, params: Result<String, ()>) -> bool;
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
      let (task, args) = next.unwrap();
      task.perform(args);
    } else {
      println!("Nothing more!");
    }
  }

  fn next_task(&self) -> Result<(&Box<Task>, Result<String, ()>), &'static str> {
    let next : redis::RedisResult<String> = self.conn.rpop(format!("{}:tasks", self.prefix));

    // TODO: it should be a better way to write this
    if next.is_ok() {
      let next_payload = next.unwrap();
      let r = self.get_task_by_name(&next_payload);
      let args : Result<String, ()>;

      if r.is_ok() {
        let task : &Box<Task> = r.unwrap();
        if next_payload.len() > task.get_name().len() {
          args = Ok(next_payload[task.get_name().len()+1 .. next_payload.len()].to_string());
        } else {
          args = Err(());
        }

        Ok((task, args))

      } else {
        Err("Error")
      }

    } else {
      Err("No more")
    }
  }

  fn get_task_by_name(&self, name: &str) -> Result<&Box<Task>, &'static str> {
    for task in &self.tasks {
      let task_name = name[0..task.get_name().len()].to_string();
      if task_name == task.get_name() {
        return Ok(task)
      }
    }
    Err("Task not found!")
  }

}

#[test]
fn it_works() {
}
