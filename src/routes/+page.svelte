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
</script>

<div class="container">
  <h1>Welcome to Tauri!</h1>
  <button on:click={click}>os</button>
  <button on:click={clickBarcode}>barcode</button>

  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>

  <p>{greetMsg}</p>
</div>
