export interface Options {
  timeout?: number | undefined | null
  cwd?: string | undefined | null
}
export interface ReturnType {
  output: string
  truncated: boolean
}
export function exec(cmd: string, opts?: Options): Promise<ReturnType>
