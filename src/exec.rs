use napi::{bindgen_prelude::*, threadsafe_function::*};
use napi_derive::napi;
use std::thread;

use crate::common;
#[cfg(not(windows))]
use crate::unix::*;
#[cfg(windows)]
use crate::windows::*;

#[napi]
pub fn exec(cmd: String, opts: common::Options, callback: JsFunction) -> Result<()> {
  let tsfn: ThreadsafeFunction<(String, bool), ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(String, bool)>| {
      ctx.env.create_object().map(|mut v| {
        v.set("output", ctx.value.0).unwrap();
        v.set("truncated", ctx.value.1).unwrap();
        vec![v]
      })
    })?;

  let data = common::Data {
    callback: Some(tsfn),
    cmd,
    options: opts,
  };

  thread::spawn(move || {
    spawn_cmd(&data);
  });

  Ok(())
}

#[napi]
pub fn exec_sync(cmd: String, opts: common::Options) -> Option<common::Result> {
  let data = common::Data {
    callback: None,
    cmd,
    options: opts,
  };

  spawn_cmd(&data)
}
