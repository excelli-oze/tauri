# Readme

- Development environment doesn't work in our selected browser, but the browser popped up when run `npm run tauri dev`
- To open `inspector` in android and ios, https://v2.tauri.app/develop/#opening-the-web-inspector-1
- before running `npm run tauri android dev` start the android emulator in android studio, so that the tauri recognizes and auto isntalls app on it
- Plugin implementation is done in rust, js implementation is optional, only if js wrapper around rust apis are needed

## Plugin Development

### Overview
1. `cargo tauri plugin new [name]` if only rust implmentation
2. `npm run tauri plugin new [name]` then if need js wrapper around rust implementation
3. `cd plugin-folder` then add plugin implementation code then `cargo build` then `npm run build` then `cd main-project` then `npm install ./path/to/pluginfolder/`

### Code Implementation
- Run `cargo build` to build inside plugin folder and then add to dependency path to `cargo.toml` in `src-tauri`
- Run `npm install` and then `npm run build` to build javascript api wrapper in `dist-js` folder and install in main project using `npm install ./path/to/pluginfolder/`
- Register custom plugin or even first party plugins in `lib.rs` before usage.
- Update `COMMANDS` in build.rs, which will update the permissions, which should get auto generated in `custom-plugin/permissions` folder. Now update in `src-tauri/capabilites`
- Add android implementation in `plugin-folder/android/src/main/java`

## Others
- when building rust compiler is able to pinpoint rust code error but isn't very helpful when android code related error
- If any android compiler error occur, mostly due some code issue within `plugin-folder/android/src/main/java` 
- Logs
  - Android - log.d() in logcat in android studio
  - Rust - println!() in terminal server
  - Web - console.log() in console inspector
  - Web in Mobile - chrome://inspect  


- WIP `a9e053e` & `948548f` commits unsuccessfull senddatatoandroid(dataobject) method