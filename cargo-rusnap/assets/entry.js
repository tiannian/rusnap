// "use strict";

const { _entry, initSync, on_rpc_request, on_transaction, on_cronjob } =
  wasm_bindgen;

function __load_wasm() {
  const byteString = atob(__wasm_module);
  const ab = new ArrayBuffer(byteString.length);
  const ia = new Uint8Array(ab);
  for (let i = 0; i < byteString.length; i++) ia[i] = byteString.charCodeAt(i);
  initSync(ia.buffer);
  if (_entry != undefined) _entry();
}
__load_wasm();

async function onRpcRequest({ origin, request }) {
  if (on_rpc_request != undefined) {
    return await on_rpc_request(origin, request.method, request.params);
  }
}
async function onTransaction({ transaction, chainId, transactionOrigin }) {
  if (on_rpc_request != undefined) {
    return await on_transaction(transaction, chainId, transactionOrigin);
  }
}
async function onCronjob({ request }) {
  if (on_cronjob != undefined) {
    return await on_cronjob(request.method, request.params);
  }
}

Object.defineProperty(exports, "__esModule", { value: true });

exports.onRpcRequest = onRpcRequest;
exports.onCronjob = onCronjob;
exports.onTransaction = onTransaction;
