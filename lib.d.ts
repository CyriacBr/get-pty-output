/* eslint-disable */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export interface Options {
  timeout?: number | undefined | null
  cwd?: string | undefined | null
}
export interface Result {
  output: string
  truncated: boolean
}
export function exec(cmd: string, opts: Options, callback: (...args: any[]) => any): void
export function execSync(cmd: string, opts: Options): any | undefined | null
