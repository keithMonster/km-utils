# @keithmonster/utils

utils for Node.js


### FsUtils
```ts
export class FsUtils {
  static rmdir(path: string): boolean | null
  static mkdir(path: string): boolean | null
  static readdir(path: string): Array<string>
  static write(path: string, contents: string): boolean
  static read(path: string): string
  static rm(path: string): boolean | null
  static isDir(path: string): boolean | null
  static isFile(path: string): boolean | null
}
```

### PathUtils
```ts
export class PathUtils {
  static basename(path: string, ext?: string | undefined | null): string
  static extname(path: string): string | null
}
```
