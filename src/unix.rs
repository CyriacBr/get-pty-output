use napi::{bindgen_prelude::*, threadsafe_function::*};
use std::io::prelude::*;
use std::io::BufReader;

use crate::common;

#[cfg(not(windows))]
pub fn spawn_cmd(data: common::Data) -> Option<common::Result> {
  use portable_pty::{native_pty_system, CommandBuilder, PtySize};
  use std::time::Instant;

  let timeout: u32 = match data.options.timeout {
    Some(v) => v,
    _ => 10,
  };
  let cwd: String = match data.options.cwd {
    Some(v) => v,
    _ => std::env::current_dir()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string(),
  };

  let pty_system = native_pty_system();
  let pair = pty_system
    .openpty(PtySize {
      rows: 24,
      cols: 80,
      pixel_width: 0,
      pixel_height: 0,
    })
    .expect("Failed to create PTY");

  let args = shellwords::split(&data.cmd).unwrap();
  let mut cmd = CommandBuilder::from_argv(args.iter().map(std::ffi::OsString::from).collect());
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
    match data.callback {
      Some(cb) => {
        cb.call(
          Ok((output, truncated)),
          ThreadsafeFunctionCallMode::Blocking,
        );
        None
      }
      _ => Some(common::Result { output, truncated }),
    }
  } else {
    match data.callback {
      Some(cb) => {
        cb.call(
          Err(Error::new(Status::Unknown, output)),
          ThreadsafeFunctionCallMode::Blocking,
        );
        None
      }
      _ => panic!("{}", output),
    }
  }
}