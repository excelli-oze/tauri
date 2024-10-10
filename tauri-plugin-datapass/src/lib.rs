use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Datapass;
#[cfg(mobile)]
use mobile::Datapass;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the datapass APIs.
pub trait DatapassExt<R: Runtime> {
    fn datapass(&self) -> &Datapass<R>;
}

impl<R: Runtime, T: Manager<R>> crate::DatapassExt<R> for T {
    fn datapass(&self) -> &Datapass<R> {
        self.state::<Datapass<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("datapass")
        .invoke_handler(tauri::generate_handler![
            commands::ping,
            commands::send_data_to_android,
            commands::get_data_from_android
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let datapass = mobile::init(app, api)?;
            #[cfg(desktop)]
            let datapass = desktop::init(app, api)?;
            app.manage(datapass);
            Ok(())
        })
        .build()
}
