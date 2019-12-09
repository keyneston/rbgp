pub mod error;
pub mod header;
pub mod message;
pub mod open;
pub mod route;
pub mod update;

pub use error::*;
pub use header::*;
pub use message::*;
pub use open::*;
pub use update::*;

///////////
// Types //
///////////

/// ASN is an autonomous system number.
type ASN = u16;
