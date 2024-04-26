/* tslint:disable */
/* eslint-disable */
/**
 * @param {number} seed
 * @param {number} x_expand_prop
 * @returns {StandardMap | undefined}
 */
export function create_standard_map(seed: number, x_expand_prop: number): StandardMap | undefined;
/**
 */
export class ElevationBuffer {
	free(): void;
	/**
	 * @param {StandardMap} standard
	 * @param {number} image_width
	 * @param {number} image_height
	 */
	constructor(standard: StandardMap, image_width: number, image_height: number);
	/**
	 * @param {number} x
	 * @param {number} y
	 * @returns {number}
	 */
	get_elevation(x: number, y: number): number;
}
/**
 */
export class MapSite {
	free(): void;
	/**
	 */
	x: number;
	/**
	 */
	y: number;
}
/**
 */
export class Name {
	free(): void;
	/**
	 * @returns {string}
	 */
	name(): string;
	/**
	 * @returns {string}
	 */
	reading(): string;
}
/**
 */
export class NameSet {
	free(): void;
	/**
	 * @returns {Name}
	 */
	city_name(): Name;
	/**
	 * @returns {Name}
	 */
	county_name(): Name;
	/**
	 * @returns {Name}
	 */
	subprefecture_name(): Name;
	/**
	 * @returns {Name}
	 */
	subprefecture_postfix(): Name;
	/**
	 * @returns {Name}
	 */
	government(): Name;
}
/**
 */
export class NetworkNode {
	free(): void;
	/**
	 * @returns {MapSite}
	 */
	site(): MapSite;
	/**
	 * @returns {number}
	 */
	stage(): number;
}
/**
 */
export class NetworkPath {
	free(): void;
}
/**
 */
export class StandardMap {
	free(): void;
	/**
	 * @param {number} seed
	 * @param {number} x_expand_prop
	 * @returns {StandardMap | undefined}
	 */
	static new(seed: number, x_expand_prop: number): StandardMap | undefined;
	/**
	 * @returns {NameSet}
	 */
	get_nameset(): NameSet;
	/**
	 * @returns {number}
	 */
	get_population(): number;
	/**
	 * @returns {MapSite}
	 */
	bound_min(): MapSite;
	/**
	 * @returns {MapSite}
	 */
	bound_max(): MapSite;
	/**
	 * @param {number} x
	 * @param {number} y
	 * @returns {number | undefined}
	 */
	get_elevation(x: number, y: number): number | undefined;
	/**
	 * @returns {(NetworkPath)[]}
	 */
	network_paths(): NetworkPath[];
	/**
	 * @returns {MapSite}
	 */
	get_origin_site(): MapSite;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
	readonly memory: WebAssembly.Memory;
	readonly __wbg_elevationbuffer_free: (a: number) => void;
	readonly elevationbuffer_from_terrain: (a: number, b: number, c: number) => number;
	readonly elevationbuffer_get_elevation: (a: number, b: number, c: number) => number;
	readonly __wbg_name_free: (a: number) => void;
	readonly name_name: (a: number, b: number) => void;
	readonly name_reading: (a: number, b: number) => void;
	readonly __wbg_nameset_free: (a: number) => void;
	readonly nameset_city_name: (a: number) => number;
	readonly nameset_county_name: (a: number) => number;
	readonly nameset_subprefecture_name: (a: number) => number;
	readonly nameset_subprefecture_postfix: (a: number) => number;
	readonly nameset_government: (a: number) => number;
	readonly __wbg_mapsite_free: (a: number) => void;
	readonly __wbg_get_mapsite_x: (a: number) => number;
	readonly __wbg_set_mapsite_x: (a: number, b: number) => void;
	readonly __wbg_get_mapsite_y: (a: number) => number;
	readonly __wbg_set_mapsite_y: (a: number, b: number) => void;
	readonly __wbg_networknode_free: (a: number) => void;
	readonly networknode_site: (a: number) => number;
	readonly networknode_stage: (a: number) => number;
	readonly __wbg_networkpath_free: (a: number) => void;
	readonly create_standard_map: (a: number, b: number) => number;
	readonly __wbg_standardmap_free: (a: number) => void;
	readonly standardmap_get_nameset: (a: number) => number;
	readonly standardmap_get_population: (a: number) => number;
	readonly standardmap_bound_min: (a: number) => number;
	readonly standardmap_bound_max: (a: number) => number;
	readonly standardmap_get_elevation: (a: number, b: number, c: number, d: number) => void;
	readonly standardmap_network_paths: (a: number, b: number) => void;
	readonly standardmap_get_origin_site: (a: number) => number;
	readonly standardmap_new: (a: number, b: number) => number;
	readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
	readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
export default function __wbg_init(
	module_or_path?: InitInput | Promise<InitInput>
): Promise<InitOutput>;
