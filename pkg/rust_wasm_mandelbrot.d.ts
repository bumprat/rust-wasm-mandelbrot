/* tslint:disable */
/* eslint-disable */
/**
* @param {number} max_iter
* @param {number} center_x
* @param {number} center_y
* @param {number} scale
* @param {number} over_sample_factor
* @param {number} color_step
* @param {number} color_number
* @param {number} color_shift
*/
export function paint(max_iter: number, center_x: number, center_y: number, scale: number, over_sample_factor: number, color_step: number, color_number: number, color_shift: number): void;
/**
* @param {number} dx
* @param {number} dy
* @param {number} dw
* @param {number} dh
*/
export function transfer(dx: number, dy: number, dw: number, dh: number): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly paint: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly transfer: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
