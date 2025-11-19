/* tslint:disable */
/* eslint-disable */
export function start(): void;
export class Game {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  static new(): Game;
  tick(): void;
  draw(ctx: CanvasRenderingContext2D): void;
  set_direction(dir_code: number): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_game_free: (a: number, b: number) => void;
  readonly game_new: () => number;
  readonly game_tick: (a: number) => void;
  readonly game_draw: (a: number, b: any) => void;
  readonly game_set_direction: (a: number, b: number) => void;
  readonly start: () => void;
  readonly wasm_bindgen__convert__closures_____invoke__h1689f7195778a2e7: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h16f1c10c00f65ce5: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__ha8d6741ed2150bb4: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
