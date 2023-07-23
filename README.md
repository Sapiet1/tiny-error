# tiny-error

A tiny crate for error handling. It is able to convert items of the 
[`Error`](https://doc.rust-lang.org/std/error/trait.Error.html) trait into
their messages, allowing for easy propagation. 

# Examples

```rust
use tiny_error::ErrorMessage;

use std::{
    env,
    fs,
    path::PathBuf,
};

fn main() -> Result<(), ErrorMessage> {
    // Text when failed:
    // Error: Invalid input
    // Correct Usage: `[crate_name] example/file/path.txt`
    let path = get_path()?;
    // Text when failed: 
    // Error: No such file or directory (os error 2)
    let file = fs::read_to_string(path)?;

    Ok(())
}

// Gets the first argument passed. If none or more were, returns an
// `ErrorMessage`.
fn get_path() -> Result<PathBuf, ErrorMessage> {
    let mut args = env::args().skip(1);
    let arg = args.next().filter(|_| args.next().is_none());    

    arg
        .map(|input| input.into())
        .ok_or_else(|| ErrorMessage::new(
            "Invalid input\n\
             Correct Usage: `[crate_name] example/file/path.txt`"
        ))
}
```
