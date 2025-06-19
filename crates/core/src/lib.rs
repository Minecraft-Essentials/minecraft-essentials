#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

pub use reqwest as HTTP;

// Modules
/// Error handling module for the Minecraft-Essentials library.
///
/// This module contains all the error types and related functionality
/// for error handling within the library.
pub mod errors;

/// Trait aliases for libraires
pub mod trait_alias;

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
pub mod auth;

#[cfg(feature = "modrinth")]
pub mod modrinth;

#[cfg(feature = "auth")]
pub use auth::AuthInfo as CustomAuthData;

// Constants
pub const EXPERIMENTAL_MESSAGE: &str =
    "\x1b[33mNOTICE: You are using an experimental feature.\x1b[0m";

#[cfg(feature = "launch")]
pub(crate) const MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[cfg(feature = "auth")]
pub(crate) const MOJANG_REDIR_URL: &str = "https://login.live.com/oauth20_desktop.srf";
