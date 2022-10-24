pub mod models;
pub mod server;

// Pull in tests as a single module so we can organize them in one folder
// but didn't want to make them integration tests since this binary crate
// doesn't expose much as public
#[cfg(test)]
mod tests;
