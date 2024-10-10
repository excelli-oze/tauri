import { invoke } from "@tauri-apps/api/core";

export async function ping(value: string): Promise<string | null> {
  return await invoke<{ value?: string }>("plugin:datapass|ping", {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export async function sendDataToAndroid(
  message: string,
  number: number
): Promise<{ success: boolean; message: string }> {
  return await invoke("plugin:datapass|sendDataToAndroid", {
    payload: {
      message,
      number,
    },
  });
}

export async function getDataFromAndroid(): Promise<{ data: string }> {
  return await invoke("plugin:datapass|getDataFromAndroid", {});
}