#![doc = include_str!("../README.md")]
#![forbid(unsafe_code, missing_docs)]
#![warn(clippy::pedantic)]

// Modules
/// Error handling module for the Minecraft-Essentials library.
///
/// This module contains all the error types and related functionality
/// for error handling within the library.
pub mod errors;
pub(crate) mod trait_alias;

/// Structs module for the Minecraft-Essentials library.
///
/// This module contains all the structs and related functionality
/// for structs within the library.
pub mod structs;
#[cfg(test)]
mod tests;

#[cfg(feature = "launch")]
/// Launch module for the Minecraft-Essentials library.
pub mod launch;

#[cfg(feature = "auth")]
mod auth;

#[cfg(feature = "modrinth")]
mod modrinth;

#[cfg(feature = "auth")]
pub use auth::AuthInfo as CustomAuthData;

// Constants
pub const EXPERIMENTAL_MESSAGE: &str =
    "\x1b[33mNOTICE: You are using an experimental feature.\x1b[0m";

#[cfg(feature = "launch")]
pub(crate) const MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";


