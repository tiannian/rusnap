import {
  _entry,
  initSync,
  on_cronjob,
  on_rpc_request,
  on_transaction,
} from "./pkg/__rusnap.js";
import * as module from "./pkg/__rusnap_bg.wasm";

function __load_wasm() {
  const byteString = atob(module.default.split(",")[1]);
  const ab = new ArrayBuffer(byteString.length);
  const ia = new Uint8Array(ab);
  for (let i = 0; i < byteString.length; i++) {
    ia[i] = byteString.charCodeAt(i);
  }

  initSync(ia.buffer);

  if (_entry != undefined) {
    _entry();
  }
}

__load_wasm();

export async function onRpcRequest({ origin, request }) {
  console.debug(request);

  if (on_rpc_request != undefined) {
    return await on_rpc_request(origin, request.method, request.params);
  }
}

export async function onTransaction({
  transaction,
  chainId,
  transactionOrigin,
}) {
  if (on_rpc_request != undefined) {
    return await on_transaction(transaction, chainId, transactionOrigin);
  }
}

export async function onCronjob({ request }) {
  if (on_cronjob != undefined) {
    return await on_cronjob(request.method, request.params);
  }
}
