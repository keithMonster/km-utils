# @keithmonster/utils

utils for Node.js


### rmDir
```ts
export function rmDir(path: string): boolean | null
```

### mkDir
```ts
export function mkDir(path: string): boolean | null
```

### readdir
```ts
export function readdir(path: string): Array<string>
```

### isDir
```ts
export function isDir(path: string): boolean | null
```

### isFile
```ts
export function isFile(path: string): boolean | null
```

### PathUtils
```ts
export class PathUtils {
  static basename(path: string, ext?: string | undefined | null): string
  static extname(path: string): string | null
}
```
