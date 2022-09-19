export interface Options {
  /**
   * Close the pty after a timeout.
   */
  timeout?: number | undefined | null
  /**
   * Close the pty after a timeout since the command became idle.
   * A command is considered idle when it has stopped writing on stdout.
   * Only works on UNIX platforms for now.
   */
  idleTimeout?: number | undefined | null
  cwd?: string | undefined | null
  /**
   * Remove cursor transformations from the output.
   * Defaults to true. You may disable this if you notice your output is weirdly truncated.
   * If the command you're executing transforms the terminal cursor (spinners, progress bars, etc),
   * you likely want to keep this to true
   */
  purify?: boolean | undefined | null
  /**
   * A callback that is invoked while the process is running, similarly to
   * `child_process.stdout.on('data', cb)`
   *
   * **Note that when using this option, no output will be returned!**
   */
  onData?: (line: string) => void
}

export interface ReturnType {
  output: string
  truncated: boolean
}
/**
 * Execute a command and return it's output
 * @param {string} cmd
 * @param {import('./index').Options} opts
 * @returns {Promise<import('./index').ReturnType>}
 */
export function exec(cmd: string, opts?: Options): Promise<ReturnType>
/**
 * Execute a command synchronously and return it's output
 * @param {string} cmd
 * @param {import('./index').Options} opts
 * @returns {import('./index').ReturnType}
 */
export function execSync(cmd: string, opts?: Options): ReturnType
