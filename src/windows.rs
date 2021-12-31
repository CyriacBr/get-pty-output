use napi::{bindgen_prelude::*, threadsafe_function::*};
use std::io::prelude::*;
use std::io::BufReader;

use crate::common;

#[cfg(windows)]
pub fn spawn_cmd(data: &common::Data) -> Option<common::Result> {
  let timeout: u32 = match data.options.timeout {
    Some(v) => v,
    _ => 10,
  };
  let cwd: String = match &data.options.cwd {
    Some(v) => String::from(v),
    _ => std::env::current_dir()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string(),
  };

  let proc_attr = conpty::ProcAttr::cmd(&data.cmd);
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
  let output = common::transform_output(&lines.join("\n"), &data.options);

  if status == 0 || truncated {
    match &data.callback {
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
    match &data.callback {
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
