use crate::models::*;
use crate::DatapassExt;
use crate::Result;
use tauri::{command, AppHandle, Runtime};

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.datapass().ping(payload)
}

#[command]
pub(crate) async fn send_data_to_android<R: Runtime>(
    app_handle: tauri::AppHandle<R>,
    text: String,
) -> Result<String> {
    app_handle.datapass().send_data_to_android(text)
}

#[command]
pub(crate) async fn get_data_from_android<R: Runtime>(
    app: AppHandle<R>,
) -> Result<GetDataResponse> {
    app.datapass().get_data_from_android()
}