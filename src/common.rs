use napi::threadsafe_function::*;
use napi_derive::napi;

pub struct Data {
  pub callback: Option<ThreadsafeFunction<(String, bool), ErrorStrategy::CalleeHandled>>,
  pub cmd: String,
  pub options: Options,
}

#[napi(object)]
pub struct Options {
  pub timeout: Option<u32>,
  pub idle_timeout: Option<u32>,
  pub cwd: Option<String>,
}

#[napi(object)]
pub struct Result {
  pub output: String,
  pub truncated: bool,
}
