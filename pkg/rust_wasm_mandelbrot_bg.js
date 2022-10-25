import * as wasm from "./rust_wasm_mandelbrot_bg.wasm";

const lTextDecoder =
  typeof TextDecoder === "undefined"
    ? (0, module.require)("util").TextDecoder
    : TextDecoder;

let cachedTextDecoder = new lTextDecoder("utf-8", {
  ignoreBOM: true,
  fatal: true,
});

cachedTextDecoder.decode();

let cachedUint8Memory0 = new Uint8Array();

function getUint8Memory0() {
  if (cachedUint8Memory0.byteLength === 0) {
    cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
  return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
/**
 */
export function run() {
  wasm.run();
}

/**
 */
export class Mandelbrot {
  __destroy_into_raw() {
    const ptr = this.ptr;
    this.ptr = 0;

    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_mandelbrot_free(ptr);
  }
}

export function __wbg_alert_38e90685f6285402(arg0, arg1) {
  alert(getStringFromWasm0(arg0, arg1));
}

export function __wbindgen_throw(arg0, arg1) {
  throw new Error(getStringFromWasm0(arg0, arg1));
}
