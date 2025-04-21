# snafu-tracing

Snafu tracing is an error handling mechanism with pseudo-stack traces implemented through SNAFU, the proc macro trace_error, and the DebugTrace trait, inspired by GreptimeDB.

## Example

```rust
use snafu::Snafu;
use snafu_tracing::{DebugTrace, trace_error, quick_tracing};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[trace_error]
#[derive(Snafu, DebugTrace)]
#[snafu(module, context(suffix(false)), visibility(pub))]
pub enum Error {
    #[snafu(display("{_error}"))]
    Any { _error: String },
    #[snafu(display("{error}"))]
    Wrap {
        error: Box<dyn std::error::Error + Send + Sync>,
    },
}

quick_tracing!(anyerr, crate::error::Any);

pub fn hello_err() -> Result<()> {
    let _e = anyerr!("Any error test! {}", 123);
    Err(anyerr!("Any error test!"))
}

fn main() {
    let e = hello_err();
    println!("{:?}", e);
}
```

Or refer to: [full example](https://github.com/dancingpeanut/snafu-tracing/tree/master/example)

## References

1. [Rust 错误处理在 GreptimeDB 的实践](https://mp.weixin.qq.com/s/PK38PtvAETD7pcHeqeDSTA)
   (Rust error handling practice in GreptimeDB). Strongly recommended. Google
   translation should be enough for non-Chinese speakers.
