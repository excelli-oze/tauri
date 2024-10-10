use tauri::{AppHandle, command, Runtime};
use crate::models::*;
use crate::Result;
use crate::DatapassExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.datapass().ping(payload)
}

#[command]
pub(crate) async fn send_data_to_android<R: Runtime>(
    app: AppHandle<R>,
    payload: SendDataRequest,
) -> Result<SendDataResponse> {
    app.datapass().send_data_to_android(payload)
}

#[command]
pub(crate) async fn get_data_from_android<R: Runtime>(
    app: AppHandle<R>,
) -> Result<GetDataResponse> {
    app.datapass().get_data_from_android()
}