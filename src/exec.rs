use napi::{bindgen_prelude::*, threadsafe_function::*};
use napi_derive::napi;
use std::thread;

use crate::common::{self, DoneThreadsafeFn, OnDataThreadsafeFn};
#[cfg(not(windows))]
use crate::unix::*;
#[cfg(windows)]
use crate::windows::*;

#[napi]
pub fn exec(cmd: String, opts: common::Options, stream_callback: Option<JsFunction>, done_callback: JsFunction) -> Result<()> {
  let data = common::Data {
    done_callback: Some(create_done_thradsafe_function(&done_callback)?),
    on_data_callback: if let Some(on_data) = stream_callback {
      Some(create_data_thradsafe_function(on_data)?)
    } else {
      None
    },
    cmd,
    options: opts,
  };

  thread::spawn(move || {
    spawn_cmd(&data);
  });

  Ok(())
}

#[napi]
pub fn exec_sync(cmd: String, opts: common::Options, stream_callback: Option<JsFunction>) -> Result<Option<common::Result>> {
  let data = common::Data {
    done_callback: None,
    on_data_callback: if let Some(on_data) = stream_callback {
      Some(create_data_thradsafe_function(on_data)?)
    } else {
      None
    },
    cmd,
    options: opts,
  };

  Ok(spawn_cmd(&data))
}

fn create_done_thradsafe_function(callback: &JsFunction) -> Result<DoneThreadsafeFn> {
  callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(String, bool)>| {
    ctx.env.create_object().map(|mut v| {
      v.set("output", ctx.value.0).unwrap();
      v.set("truncated", ctx.value.1).unwrap();
      vec![v]
    })
  })
}


fn create_data_thradsafe_function(callback: JsFunction) -> Result<OnDataThreadsafeFn> {
  callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<String>| {
    ctx.env.create_string(&ctx.value).map(|v| vec![v])
  })
}