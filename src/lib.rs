#![deny(clippy::all)]

use napi::{bindgen_prelude::*, threadsafe_function::*};
use napi_derive::napi;
use pty::fork::Fork;
use std::io::Read;
use std::process::Command;
use std::time::Instant;

#[napi(object)]
pub struct Options {
  pub timeout: Option<u32>,
  pub idle_timeout: Option<u32>,
}

#[napi]
pub fn exec(cmd: String, opts: Option<Options>, callback: JsFunction) -> Result<()> {
  let maybe_opts = opts.as_ref().unwrap();
  let timeout: u32 = match maybe_opts.timeout {
    Some(v) => v,
    _ => 10,
  };
  let idle_timeout: u32 = match maybe_opts.idle_timeout {
    Some(v) => v,
    _ => 2,
  };

  let tsfn: ThreadsafeFunction<String, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx| {
      ctx.env.create_string_from_std(ctx.value).map(|v| vec![v])
    })?;

  std::thread::spawn(move || {
    let fork = Fork::from_ptmx().unwrap();
    let args = shellwords::split(&cmd).unwrap();

    if let Some(mut master) = fork.is_parent().ok() {
      let mut output = String::new();
      let now = Instant::now();
      let mut truncated = false;

      // let mut read_bytes: u32 = 0;
      for byte in master.bytes() {
        // read_bytes += 1;
        output += std::str::from_utf8(&vec![byte.unwrap()]).unwrap();
        // if (read_bytes >= 10) {
        //     read_bytes = 0;
        if now.elapsed().as_secs() >= (timeout as u64) {
          truncated = true;
          break;
        }
        // }
      }

      tsfn.call(Ok(output), ThreadsafeFunctionCallMode::Blocking);
    } else {
      Command::new(&args[0])
        .args(&args[1..])
        .status()
        .expect("could not execute command");
    }
  });
  Ok(())
}
