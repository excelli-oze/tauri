use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Datapass<R>> {
    Ok(Datapass(app.clone()))
}

/// Access to the datapass APIs.
pub struct Datapass<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Datapass<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }

    pub fn send_data_to_android(
        &self,
        payload: SendDataRequest,
    ) -> crate::Result<SendDataResponse> {
        let message = payload.message.clone();
        let number = payload.number;

        let response_message = format!("Received message: '{}' with number: {}", message, number);

        Ok(SendDataResponse {
            value: Some(response_message),
        })
    }

    pub fn get_data_from_android(&self) -> crate::Result<GetDataResponse> {
        // Indicate that desktop is not supported
        Ok(GetDataResponse {
            data: "Desktop is not supported. Please use mobile.".to_string(),
        })
    }
}
