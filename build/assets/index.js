import { initSync } from "./pkg/__rusnap.js";
import * as module from "./pkg/__rusnap_bg.wasm";

function __load_wasm() {
  const byteString = atob(module.default.split(",")[1]);
  const ab = new ArrayBuffer(byteString.length);
  const ia = new Uint8Array(ab);
  for (let i = 0; i < byteString.length; i++) {
    ia[i] = byteString.charCodeAt(i);
  }

  initSync(ia.buffer);
}

__load_wasm();

export * from "./pkg/__rusnap.js";
