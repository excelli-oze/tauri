import { invoke } from "@tauri-apps/api/core";

export async function ping(value: string): Promise<string | null> {
  return await invoke<{ value?: string }>("plugin:datapass|ping", {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export async function sendDataToAndroid(data: {
  message: string;
  number: number;
}): Promise<string | null> {
  return await invoke<{ value?: string }>("plugin:datapass|sendDataToAndroid", {
    payload: data,
  }).then((r) => (r.value ? r.value : null));
}

export async function getDataFromAndroid(): Promise<{ data: string }> {
  return await invoke("plugin:datapass|getDataFromAndroid", {});
}
