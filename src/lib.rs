#![deny(clippy::all)]

use napi::{bindgen_prelude::*, threadsafe_function::*};
use napi_derive::napi;
use std::io::prelude::*;
use std::io::BufReader;

#[napi(object)]
pub struct Options {
  pub timeout: Option<u32>,
  pub cwd: Option<String>,
}

#[cfg(not(windows))]
#[napi]
pub fn exec(cmd: String, opts: Options, callback: JsFunction) -> Result<()> {
  use portable_pty::{native_pty_system, CommandBuilder, NativePtySystem, PtySize, PtySystem};
  use std::time::Instant;

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
  let pair = pty_system
    .openpty(PtySize {
      rows: 24,
      cols: 80,
      pixel_width: 0,
      pixel_height: 0,
    })
    .expect("Failed to create PTY");

  let args = shellwords::split(&cmd).unwrap();
  let mut cmd = CommandBuilder::from_argv(args.iter().map(std::ffi::OsString::from).collect());
  cmd.cwd(cwd);

  let mut child = pair
    .slave
    .spawn_command(cmd)
    .expect("Failed to spawn command");
  child.wait().unwrap();
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
      Err(Error::new(Status::Unknown, output)),
      ThreadsafeFunctionCallMode::Blocking,
    );
  }

  Ok(())
}

#[cfg(windows)]
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

  let proc_attr = conpty::ProcAttr::cmd(cmd);
  let proc_attr = proc_attr.current_dir(cwd);
  let proc = proc_attr.spawn().unwrap();
  let mut raw_reader = proc.output().unwrap();
  raw_reader.set_non_blocking_mode().unwrap();

  let mut truncated = false;
  let status = match proc.wait(Some(timeout * 1000)) {
    Ok(v) => v,
    _ => {
      truncated = true;
      1
    }
  };
  let mut lines = Vec::<String>::new();
  let reader = BufReader::new(raw_reader);
  for ln in reader.lines() {
    match ln {
      Ok(v) => lines.push(v),
      _ => break,
    }
  }
  let output = lines.join("\n");

  if status == 0 || truncated {
    tsfn.call(
      Ok((output, truncated)),
      ThreadsafeFunctionCallMode::Blocking,
    );
  } else {
    tsfn.call(
      Err(Error::new(Status::Unknown, output)),
      ThreadsafeFunctionCallMode::Blocking,
    );
  }

  Ok(())
}
