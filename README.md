# PathBufD

`PathBuf` but with `Display` and a macro similar to `format!` for formatting.

Implements every (stable) API from [`PathBuf`](https://doc.rust-lang.org/std/path/struct.PathBuf.html) + some extras.

## Usage

Creating a `PathBufD` in the current directory:

```rust
use pathbufd::{PathBufD, format_path};

fn main() {
    let buf = PathBufD::current();
    println!("path: {buf}")
}
```

Creating a `PathBufD` and pushing to it:

```rust
use pathbufd::{PathBufD};

fn main() {
    // create a new pathbuf
    let mut buf = PathBufD::new();

    // push to buf
    buf.push("directory");
    buf.push("file");

    // print result
    println!("path: {buf}")
}
```

Creating a `PathBufD` and joining to it:

```rust
use pathbufd::{PathBufD};

fn main() {
    // create a new pathbuf
    let buf = PathBufD::new().join("directory").join("file");

    // print result
    println!("path: {buf}")
}
```

Creating a `PathBufD` with a formatting macro:

```rust
use pathbufd::{PathBufD, format_path};

fn main() {
    let buf = path!("{}/file", "directory");
    println!("path: {buf}")
}
```

Extend a `PathBufD` with a slice of paths:

```rust
use pathbufd::{PathBufD, format_path};

fn main() {
    let buf = PathBufD::new().extend(["directory", "file"]);
    println!("path: {buf}")
}
```
