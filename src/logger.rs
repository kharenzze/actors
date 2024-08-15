use actix::prelude::*;

pub struct Logger {}

impl Actor for Logger {
  type Context = Context<Self>;
}
