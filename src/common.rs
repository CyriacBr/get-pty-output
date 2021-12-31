use napi::threadsafe_function::*;
use napi_derive::napi;
use regex::Regex;

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
  pub purify: Option<bool>,
}

#[napi(object)]
pub struct Result {
  pub output: String,
  pub truncated: bool,
}

pub fn transform_output(output: &str, opts: &Options) -> String {
  match opts.purify {
    Some(true) | None => {
      /*
       * handle clear line/screen ANSI codes:
       * \x1B[{n}K
       * \x1B[{n}J
       */
      let result = Regex::new(r".*\x1B\[[12][KJ]")
        .unwrap()
        .replace_all(output, "")
        .to_string();
      /*
       * handle cursor movement ANSI codes:
       * \x1B[?25h
       * \x1B[{n}[ABCDEFG]
       * \x1B[{n};{m}H
       */
      let result = Regex::new(r"(\x1B\[\?25h)|(\x1B\[\d[ABCDEFG])|(\x1B\[\d;\dH)")
        .unwrap()
        .replace_all(&result, "")
        .to_string();
      result
    }
    Some(false) => String::from(output),
  }
}
