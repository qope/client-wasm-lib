import { expose } from "comlink";

async function proveSign(
  signInput: string,
  numThread: number
): Promise<string> {
  const multiThread = await import("wasm-client");
  await multiThread.default();
  await multiThread.initThreadPool(numThread);
  const ret = multiThread.proveSimpleSignature(signInput);
  return ret;
}

async function proveTx(txInput: string, numThread: number): Promise<string> {
  const multiThread = await import("wasm-client");
  await multiThread.default();
  await multiThread.initThreadPool(numThread);
  const ret = multiThread.proveUserTransaction(txInput);
  return ret;
}

const exports = {
  proveSign,
  proveTx,
};
export type WasmWorker = typeof exports;

expose(exports);
