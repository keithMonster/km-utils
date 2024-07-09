# @keithmonster/utils

utils for Node.js


### FsUtils
```ts
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
```

### PathUtils
```ts
export class PathUtils {
  static basename(path: string, ext?: string | undefined | null): string
  static extname(path: string): string | null
}
```

