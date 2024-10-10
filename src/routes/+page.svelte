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
  import {
    ping,
    getDataFromAndroid,
    sendDataToAndroid,
  } from "tauri-plugin-datapass-api";

  async function testPing() {
    try {
      const pingResult = await ping("Hello, Tauri!");
      console.log("Ping result: web to rust to android, the back to rust to web", pingResult);
    } catch (error) {
      console.error("Error testing ping:", error);
    }
  }

  // Function to send data to Android
  async function sendData() {
    try {
      const dataToSend = { message: "Hello from Web to Android", number: 42 };
      const sendResult = await sendDataToAndroid(
        dataToSend.message,
        dataToSend.number
      );
      console.log("Send result:", sendResult);
    } catch (error) {
      console.error("Error sending data to Android:", error);
    }
  }

  // Function to get data from Android
  async function getData() {
    try {
      const receivedData = await getDataFromAndroid();
      console.log("Received data:", receivedData);
    } catch (error) {
      console.error("Error getting data from Android:", error);
    }
  }
</script>

<div class="container">
  <h1>Welcome to Tauri!</h1>

  <button on:click={click}>Check OS</button>
  <button on:click={clickBarcode}>Scan Barcode</button>
  <button on:click={clickLocation}>Get Location</button>
  <button on:click={clickNotification}>Send Notification</button>

  <button on:click={testPing}>Ping Data</button>
  <button on:click={getData}>Get Data to android</button>
  <button on:click={sendData}>Send Data to android</button>

  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>

  <p>{greetMsg}</p>
</div>

<style>
  .container {
    max-width: 600px;
    margin: 0 auto;
    padding: 20px;
    font-family: Arial, sans-serif;
  }

  h1 {
    text-align: center;
  }

  button {
    display: block;
    width: 100%;
    padding: 10px;
    margin: 10px 0;
    border: none;
    border-radius: 5px;
    background-color: #007bff;
    color: white;
    font-size: 16px;
    cursor: pointer;
    transition: background-color 0.3s;
  }

  button:hover {
    background-color: #0056b3;
  }

  input {
    width: calc(100% - 22px);
    padding: 10px;
    margin: 10px 0;
    border: 1px solid #ccc;
    border-radius: 5px;
  }

  p {
    text-align: center;
    font-size: 18px;
  }
</style>
