const COMMANDS: &[&str] = &["ping", "getDataFromAndroid", "sendDataToAndroid"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
