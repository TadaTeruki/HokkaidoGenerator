/* tslint:disable */
/* eslint-disable */
/**
*/
export class Name {
  free(): void;
}
/**
*/
export class NameSet {
  free(): void;
}
/**
*/
export class NetworkNode {
  free(): void;
/**
* @returns {number}
*/
  x(): number;
/**
* @returns {number}
*/
  y(): number;
/**
* @returns {number}
*/
  stage(): number;
}
/**
*/
export class StandardMap {
  free(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_name_free: (a: number) => void;
  readonly __wbg_nameset_free: (a: number) => void;
  readonly __wbg_standardmap_free: (a: number) => void;
  readonly __wbg_networknode_free: (a: number) => void;
  readonly networknode_x: (a: number) => number;
  readonly networknode_y: (a: number) => number;
  readonly networknode_stage: (a: number) => number;
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
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
