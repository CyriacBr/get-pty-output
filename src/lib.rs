#![deny(clippy::all)]

use napi::{bindgen_prelude::*, threadsafe_function::*};
use napi_derive::napi;
use portable_pty::{native_pty_system, CommandBuilder, PtySize, PtySystem};
use pty::fork::Fork;
use std::ffi::OsStr;
use std::process::{Command, Stdio};
use std::time::Instant;

use std::io::prelude::*;
use std::io::BufReader;

#[napi(object)]
pub struct Options {
  pub timeout: Option<u32>,
  pub idle_timeout: Option<u32>,
}

#[napi]
pub fn exec(cmd: String, opts: Options, callback: JsFunction) -> Result<()> {
  let timeout: u32 = match opts.timeout {
    Some(v) => v,
    _ => 10,
  };

  let tsfn: ThreadsafeFunction<(String, bool), ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(String, bool)>| {
      ctx.env.create_object().map(|mut v| {
        v.set("output", ctx.value.0).unwrap();
        v.set("truncated", ctx.value.1).unwrap();
        vec![v]
      })
    })?;

  std::thread::spawn(move || {
    let fork = Fork::from_ptmx().unwrap();
    let args = shellwords::split(&cmd).unwrap();

    if let Some(master) = fork.is_parent().ok() {
      let now = Instant::now();
      let mut truncated = false;

      let mut lines = Vec::<String>::new();
      let reader = BufReader::new(master);

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

      tsfn.call(
        Ok((output, truncated)),
        ThreadsafeFunctionCallMode::Blocking,
      );
    } else {
      Command::new(&args[0])
        .args(&args[1..])
        .status()
        .expect("could not execute command");
    }
  });
  Ok(())
}

#[napi]
pub fn exec2(cmd: String, opts: Options, callback: JsFunction) -> Result<()> {
  let timeout: u32 = match opts.timeout {
    Some(v) => v,
    _ => 10,
  };
  let idle_timeout: u32 = match opts.idle_timeout {
    Some(v) => v,
    _ => 2,
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
  cmd.cwd(std::env::current_dir().unwrap().to_str().unwrap());

  let mut child = pair
    .slave
    .spawn_command(cmd)
    .expect("Failed to spawn command");
  drop(pair.slave);

  let box_reader = pair
    .master
    .try_clone_reader()
    .expect("Failed to get reader");

  let mut box_writer = pair
    .master
    .try_clone_writer()
    .expect("Failed to get reader");

  child.wait().expect("Couldn't wait");

  // box_writer.write_all(b"some bytes\n").expect("Couldn't write");
  // box_writer.flush().expect("Couldn't flush");
  // child.wait().expect("Couldn't wait");

  // Send data to the pty by writing to the master
  writeln!(pair.master, "I love node\r\n").expect("Couldn't write to master");
  pair.master.flush()?;
  child.wait().expect("Couldn't wait");

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

  tsfn.call(
    Ok((output, truncated)),
    ThreadsafeFunctionCallMode::Blocking,
  );

  Ok(())
}

use ptyprocess::PtyProcess;
use std::io::{Read, Write};

#[napi]
pub fn exec3(cmd: String, opts: Options, callback: JsFunction) -> Result<()> {
  let timeout: u32 = match opts.timeout {
    Some(v) => v,
    _ => 10,
  };
  let idle_timeout: u32 = match opts.idle_timeout {
    Some(v) => v,
    _ => 2,
  };

  let tsfn: ThreadsafeFunction<(String, bool), ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(String, bool)>| {
      ctx.env.create_object().map(|mut v| {
        v.set("output", ctx.value.0).unwrap();
        v.set("truncated", ctx.value.1).unwrap();
        vec![v]
      })
    })?;

  let args = shellwords::split(&cmd).unwrap();
  // spawn a cat process
  let mut cmd = Command::new(&args[0]);
  cmd.args(&args[1..]);
  // cmd.args(vec!["-c", "yn=input('Agree?'); print('answer was ' + yn)"]);
  let mut process = PtyProcess::spawn(cmd).expect("failed to spawn a process");

  // create a communication stream
  let stream = process.get_pty_stream().expect("failed to create a stream");

  let now = Instant::now();
  let mut truncated = false;
  let mut lines = Vec::<String>::new();
  let reader = BufReader::new(stream);

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

  // stop the process
  assert!(process.exit(true).expect("failed to stop the process"));

  tsfn.call(Ok((output, truncated)), ThreadsafeFunctionCallMode::Blocking);

  Ok(())
}
