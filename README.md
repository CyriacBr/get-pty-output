# `get-pty-output`

![https://github.com/CyriacBr/get-pty-output/actions](https://github.com/CyriacBr/get-pty-output/workflows/CI/badge.svg)

> An alternative to `child_process.exec` with PTY support

Node doesn't natively support creating pseudo-terminals. When running an external command
with `child_process.spawn` or `exec`, ansi colors are lost because most of CLIs disable colors when they aren't run inside a TTY environment.
You can make your child inherit the parent `stdio`, thus allowing colors, but this results in the inability to properly capture outputs.

Some packages like `node-pty` aim to solve this problem, but they rely on `node-gyp`, and doesn't support newer Node versions.
This package leverage Rust's `portable-pty`(unix) and `conpty`(windows) crates with [n-api-rs](https://github.com/napi-rs/napi-rs) for bindings, allowing way better
cross-platform support and easier installation.

## Install

```
yarn add get-pty-output
```

## Usage

```ts
import { exec, execSync } from 'get-pty-output'

const res = await exec('node -e console.log(100)')
res.output // in color! ✨
```

### Options

```ts
exec(cmd, {
  timeout: 10, //output will be truncated after 10 seconds, regardless of whether the child finished or not
  cwd: 'path-to-desired-wd',
})
```

## Publishing

* Remove all optional deps
* Bump version if needed
* Run `prepare-release`
* Push or manually start CI workflow
* Download CI artifacts and put them under ./npm in their respective directory
* Run `npm publish`

## Credits & Thanks

- [n-api-rs](https://github.com/napi-rs/napi-rs)
- [portable-pty](https://docs.rs/portable-pty/latest/portable_pty/)
- [conpty](https://github.com/zhiburt/conpty)
