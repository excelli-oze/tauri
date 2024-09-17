use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime, State,
};
use std::sync::Mutex;

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

// Shared data structure
struct SharedData(Mutex<String>);

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the datapass APIs.
pub trait DatapassExt<R: Runtime> {
    fn datapass(&self) -> &Datapass<R>;
}

impl<R: Runtime, T: Manager<R>> crate::DatapassExt<R> for T {
    fn datapass(&self) -> &Datapass<R> {
        self.state::<Datapass<R>>().inner()
    }
}

// Command to set data
#[tauri::command]
fn set_data(data: String, state: State<SharedData>) -> Result<()> {
    let mut shared_data = state.0.lock()?;
    *shared_data = data;
    Ok(())
}

#[tauri::command]
fn get_data(state: State<SharedData>) -> Result<String> {
    let shared_data = state.0.lock()?;
    Ok(shared_data.clone())
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("datapass")
        .invoke_handler(tauri::generate_handler![commands::ping, set_data, get_data])
        .setup(|app, api| {
            app.manage(SharedData(Mutex::new(String::new())));
            #[cfg(mobile)]
            let datapass = mobile::init(app, api)?;
            #[cfg(desktop)]
            let datapass = desktop::init(app, api)?;
            app.manage(datapass);
            Ok(())
        })
        .build()
}