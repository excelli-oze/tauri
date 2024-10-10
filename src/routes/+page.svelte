<script>
  import { invoke } from "@tauri-apps/api/core";

  let name = "";
  let greetMsg = "";

  async function greet() {
    greetMsg = await invoke("greet", { name });
  }

  // os
  import { platform } from "@tauri-apps/plugin-os";
  let currentPlatform;

  async function click() {
    currentPlatform = await platform();
    console.log("Platform", currentPlatform);
  }

  //barcode scanner
  import { scan, Format } from "@tauri-apps/plugin-barcode-scanner";
  async function clickBarcode() {
    currentPlatform = await platform();
    if (currentPlatform === "android") {
      console.log("scan button clicked");
      scan({ windowed: true, formats: [Format.QRCode] });
    } else {
      console.log("barcode scanner only available in android and ios");
    }
  }

  //location
  import {
    checkPermissions,
    requestPermissions,
    getCurrentPosition,
    watchPosition,
  } from "@tauri-apps/plugin-geolocation";

  async function clickLocation() {
    let permissions = await checkPermissions();
    if (
      permissions.location === "prompt" ||
      permissions.location === "prompt-with-rationale"
    ) {
      permissions = await requestPermissions(["location"]);
    }

    if (permissions.location === "granted") {
      const pos = await getCurrentPosition();

      await watchPosition(
        { enableHighAccuracy: true, timeout: 10000, maximumAge: 0 },
        (pos) => {
          console.log(pos);
        }
      );
    }
  }

  import {
    isPermissionGranted,
    requestPermission,
    sendNotification,
  } from "@tauri-apps/plugin-notification";

  async function clickNotification() {
    let permissionGranted = await isPermissionGranted();

    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === "granted";
    }

    if (permissionGranted) {
      sendNotification({
        title: "Test Notification",
        body: "Notification body!",
      });
    }
  }

  //datapass
  import { ping } from "tauri-plugin-datapass-api";

  async function clickDataPass() {
    try {
      const result = await ping("Hello, Tauri!");
      console.log("Ping result:", result);
    } catch (error) {
      console.error("Error calling ping:", error);
    }
  }
</script>

<div class="container">
  <h1>Welcome to Tauri!</h1>
  <button on:click={click}>os</button>
  <button on:click={clickBarcode}>barcode</button>
  <button on:click={clickLocation}>location</button>
  <button on:click={clickNotification}>Notification</button>
  <button on:click={clickDataPass}>Datapass</button>

  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>

  <p>{greetMsg}</p>
</div>
