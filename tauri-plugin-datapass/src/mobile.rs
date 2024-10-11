use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_datapass);

pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Datapass<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.datapass", "ExamplePlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_datapass)?;
    Ok(Datapass(handle))
}

pub struct Datapass<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Datapass<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        self.0
            .run_mobile_plugin("ping", payload)
            .map_err(Into::into)
    }

    pub fn send_data_to_android(
        &self,
        payload: SendDataRequest,
    ) -> crate::Result<SendDataResponse> {
        println!("mobile.rs Rust received: {:?}", payload);
        self.0
            .run_mobile_plugin("sendDataToAndroid", payload)
            .map_err(Into::into)
    }

    pub fn get_data_from_android(&self) -> crate::Result<GetDataResponse> {
        self.0
            .run_mobile_plugin("getDataFromAndroid", ())
            .map_err(Into::into)
    }
}
