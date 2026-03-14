pub use error::{Error, Result};
use tauri::{plugin::TauriPlugin, AppHandle, Manager, Runtime};

pub use minecraft_essentials::CustomAuthData;

mod commands;
mod error;

/// Access to the minecraft-essentails APIs.
pub struct MinecraftEssentails<R: Runtime>(AppHandle<R>);

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the minecraft-essentails APIs.
pub trait MinecraftEssentailsExt<R: Runtime> {
    fn minecraft_essentails(&self) -> &MinecraftEssentails<R>;
}

impl<R: Runtime, T: Manager<R>> crate::MinecraftEssentailsExt<R> for T {
    fn minecraft_essentails(&self) -> &MinecraftEssentails<R> {
        self.state::<MinecraftEssentails<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new("minecraft-essentails")
        .invoke_handler(tauri::generate_handler![commands::oauth])
        .build()
}
