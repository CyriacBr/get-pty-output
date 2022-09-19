/* eslint-disable */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export interface Options {
  timeout?: number
  idleTimeout?: number
  cwd?: string
  purify?: boolean
}
export interface Result {
  output: string
  truncated: boolean
}
export function exec(cmd: string, opts: Options, streamCallback: (...args: any[]) => any | undefined | null, doneCallback: (...args: any[]) => any): void
export function execSync(cmd: string, opts: Options, streamCallback?: (...args: any[]) => any | undefined | null): any | null
