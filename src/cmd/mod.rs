mod add;
mod import;
mod init;
mod query;
mod remove;

use anyhow::Result;

pub use add::Add;
pub use import::Import;
pub use init::Init;
pub use query::Query;
pub use remove::Remove;

pub trait Cmd {
    fn run(&self) -> Result<()>;
}
