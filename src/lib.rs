
// Modules
mod plugin;
mod types;
mod systems;

// WHAT THE PUBLIC GET TO USE
// Everything below is what becomes available for the public to use when "cargo add" gets used
// on the package.  I am not allowing the usage of the systems since I'm letting the TimeStructures
// plugin handle that - there is no need to call the systems; I'd even say it's a little dangerous
// to use the systems if you want the types to work reliably.  For anybody that wants more or disagrees
// with my opinion, go to the GitHub repo and pull whatever you want.
pub use plugin::TimeStructures;
pub use types::*;
