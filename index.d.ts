/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface BuildConfig {
  ignore: Array<string>
}
export interface UserConfig {
  name: string
  build: BuildConfig
}
export function replaceText(original: string, pattern: string, replacement: string): string
export class CliUtils {
  static clear(): void
  static preBuild(userConfig: UserConfig): void
  static compileScript(path: string, compileType: string): void
}
export class FsUtils {
  static rmdir(path: string): boolean
  static mkdir(path: string): boolean
  static readdir(path: string): Array<string>
  static write(path: string, contents: string): boolean
  static read(path: string): string
  static rm(path: string): boolean
  static isDir(path: string): boolean
  static isFile(path: string): boolean
}
export class PathUtils {
  static basename(path: string, ext?: string | undefined | null): string
  static extname(path: string): string | null
}
