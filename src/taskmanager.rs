use actix::prelude::*;

pub struct Task {
  id: u64,
  description: String,
}

pub struct TaskManager {
  tasks: Vec<Task>,
  workers: Vec<Addr<Worker>>,
}

impl Actor for TaskManager {
  type Context = Context<Self>;
}

pub struct Worker {
  id: u64,
  task: Task,
}

impl Actor for Worker {
  type Context = Context<Self>;
}
