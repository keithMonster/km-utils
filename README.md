utils for Node.js


### rmDir
```ts
export function rmDir(path: string): boolean | null
```
```rust
/// Removes a directory at the specified path.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path of the directory to be removed.
///
/// # Returns
///
/// * `Some(true)` - If the directory is successfully removed.
/// * `None` - If an error occurs during directory removal.
///
```

### mkDir
```ts
export function mkDir(path: string): boolean | null
```
```rust
/// Creates a directory at the specified path.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path of the directory to be created.
///
/// # Returns
///
/// * `Some(true)` - If the directory is successfully created.
/// * `None` - If an error occurs during directory creation.
///
```
