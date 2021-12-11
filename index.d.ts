/* eslint-disable */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export interface Options {
  timeout?: number | undefined | null
  idleTimeout?: number | undefined | null
}
export function exec(cmd: string, opts: Options, callback: (...args: any[]) => any): void
export function exec2(cmd: string, opts: Options, callback: (...args: any[]) => any): void
export function exec3(cmd: string, opts: Options, callback: (...args: any[]) => any): void
