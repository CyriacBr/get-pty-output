#![deny(clippy::all)]

use napi::{bindgen_prelude::*, threadsafe_function::*};
use napi_derive::napi;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

#[napi(object)]
pub struct Options {
  pub timeout: Option<u32>,
  pub cwd: Option<String>,
}

#[napi]
pub fn exec(cmd: String, opts: Options, callback: JsFunction) -> Result<()> {
  let timeout: u32 = match opts.timeout {
    Some(v) => v,
    _ => 10,
  };
  let cwd: String = match opts.cwd {
    Some(v) => v,
    _ => std::env::current_dir()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string(),
  };

  let tsfn: ThreadsafeFunction<(String, bool), ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(String, bool)>| {
      ctx.env.create_object().map(|mut v| {
        v.set("output", ctx.value.0).unwrap();
        v.set("truncated", ctx.value.1).unwrap();
        vec![v]
      })
    })?;

  let pty_system = native_pty_system();
  let mut pair = pty_system
    .openpty(PtySize {
      rows: 24,
      cols: 80,
      pixel_width: 0,
      pixel_height: 0,
    })
    .expect("Failed to create PTY");
  print!("pty created");

  let args = shellwords::split(&cmd).unwrap();
  let mut cmd =
    CommandBuilder::from_argv(args.iter().map(|v| std::ffi::OsString::from(v)).collect());
  cmd.cwd(cwd);

  let mut child = pair
    .slave
    .spawn_command(cmd)
    .expect("Failed to spawn command");
  drop(pair.slave);

  let box_reader = pair
    .master
    .try_clone_reader()
    .expect("Failed to get reader");
  drop(pair.master);

  let now = Instant::now();
  let mut truncated = false;
  let mut lines = Vec::<String>::new();
  let reader = BufReader::new(box_reader);

  for ln in reader.lines() {
    match ln {
      Ok(v) => lines.push(v),
      _ => break,
    }
    if now.elapsed().as_secs() >= (timeout as u64) {
      truncated = true;
      break;
    }
  }

  let output = lines.join("\n");
  let status = child.wait().unwrap();

  if status.success() || truncated {
    tsfn.call(
      Ok((output, truncated)),
      ThreadsafeFunctionCallMode::Blocking,
    );
  } else {
    tsfn.call(
      Err(Error::new(Status::Unknown, output.to_owned())),
      ThreadsafeFunctionCallMode::Blocking,
    );
  }

  Ok(())
}
