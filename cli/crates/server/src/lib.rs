/*!
The server crate provides a server with the gateway worker (via miniflare)
and a bridge server connecting the worker to an sqlite db

# Example

```ignore
const PORT: u16 = 4000;
# common::environment::Environment::try_init().unwrap();

// `common::environment::Environment` must be initialized before this

let server_handle = server::start(PORT).unwrap();
```
*/

// TODO: make the prior example testable

#![forbid(unsafe_code)]

#[macro_use]
extern crate log;

mod bridge;
mod consts;
pub mod errors;
mod servers;
pub mod types;

pub use servers::start;
