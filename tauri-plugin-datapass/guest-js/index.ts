import { invoke } from "@tauri-apps/api/core";

export async function ping(value: string): Promise<string | null> {
  return await invoke<{ value?: string }>("plugin:datapass|ping", {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export async function sendDataToAndroid(text: string): Promise<string> {
  return await invoke("plugin:datapass|sendDataToAndroid", { text });
}

export async function getDataFromAndroid(): Promise<{ data: string }> {
  return await invoke("plugin:datapass|getDataFromAndroid", {});
}
