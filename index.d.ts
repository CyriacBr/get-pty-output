export interface Options {
  timeout?: number | undefined | null
  cwd?: string | undefined | null
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
