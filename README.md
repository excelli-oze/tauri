# Readme

- Development environment doesn't work in our selected browser, but the browser popped up when run `npm run tauri dev`
- To open `inspector` in android and ios, https://v2.tauri.app/develop/#opening-the-web-inspector-1 
- before running `npm run tauri android dev` start the android emulator in android studio, so that the tauri recognizes and auto isntalls app on it
- Plugin implementation is done in rust, js implementation is optional, only if js wrapper around rust apis are needed
  
## Plugin Development
- Run `cargo build` to build and then add to dependency path to `cargo.toml` in `src-tauri`
- Run `npm install` and then `npm run build` to build javascript api wrapper in `dist-js` folder and install in main project using `npm install ./path/to/pluginfolder/`
- 