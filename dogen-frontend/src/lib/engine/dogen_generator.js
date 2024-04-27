let wasm;

const cachedTextDecoder =
	typeof TextDecoder !== 'undefined'
		? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true })
		: {
				decode: () => {
					throw Error('TextDecoder not available');
				}
			};

if (typeof TextDecoder !== 'undefined') {
	cachedTextDecoder.decode();
}

let cachedUint8Memory0 = null;

function getUint8Memory0() {
	if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
		cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
	}
	return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
	ptr = ptr >>> 0;
	return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
/**
 * @param {number} seed
 * @param {number} x_expand_prop
 * @returns {StandardMap | undefined}
 */
export function create_standard_map(seed, x_expand_prop) {
	const ret = wasm.create_standard_map(seed, x_expand_prop);
	return ret === 0 ? undefined : StandardMap.__wrap(ret);
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
	if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
		cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
	}
	return cachedInt32Memory0;
}

let cachedFloat64Memory0 = null;

function getFloat64Memory0() {
	if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
		cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
	}
	return cachedFloat64Memory0;
}

let cachedUint32Memory0 = null;

function getUint32Memory0() {
	if (cachedUint32Memory0 === null || cachedUint32Memory0.byteLength === 0) {
		cachedUint32Memory0 = new Uint32Array(wasm.memory.buffer);
	}
	return cachedUint32Memory0;
}

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) {
	return heap[idx];
}

let heap_next = heap.length;

function dropObject(idx) {
	if (idx < 132) return;
	heap[idx] = heap_next;
	heap_next = idx;
}

function takeObject(idx) {
	const ret = getObject(idx);
	dropObject(idx);
	return ret;
}

function getArrayJsValueFromWasm0(ptr, len) {
	ptr = ptr >>> 0;
	const mem = getUint32Memory0();
	const slice = mem.subarray(ptr / 4, ptr / 4 + len);
	const result = [];
	for (let i = 0; i < slice.length; i++) {
		result.push(takeObject(slice[i]));
	}
	return result;
}

function _assertClass(instance, klass) {
	if (!(instance instanceof klass)) {
		throw new Error(`expected instance of ${klass.name}`);
	}
	return instance.ptr;
}

function addHeapObject(obj) {
	if (heap_next === heap.length) heap.push(heap.length + 1);
	const idx = heap_next;
	heap_next = heap[idx];

	heap[idx] = obj;
	return idx;
}

const ElevationBufferFinalization =
	typeof FinalizationRegistry === 'undefined'
		? { register: () => {}, unregister: () => {} }
		: new FinalizationRegistry((ptr) => wasm.__wbg_elevationbuffer_free(ptr >>> 0));
/**
 */
export class ElevationBuffer {
	__destroy_into_raw() {
		const ptr = this.__wbg_ptr;
		this.__wbg_ptr = 0;
		ElevationBufferFinalization.unregister(this);
		return ptr;
	}

	free() {
		const ptr = this.__destroy_into_raw();
		wasm.__wbg_elevationbuffer_free(ptr);
	}
	/**
	 * @param {StandardMap} standard
	 * @param {number} image_width
	 * @param {number} image_height
	 */
	constructor(standard, image_width, image_height) {
		_assertClass(standard, StandardMap);
		const ret = wasm.elevationbuffer_from_terrain(standard.__wbg_ptr, image_width, image_height);
		this.__wbg_ptr = ret >>> 0;
		return this;
	}
	/**
	 * @param {number} x
	 * @param {number} y
	 * @returns {number}
	 */
	get_elevation(x, y) {
		const ret = wasm.elevationbuffer_get_elevation(this.__wbg_ptr, x, y);
		return ret;
	}
}

const MapSiteFinalization =
	typeof FinalizationRegistry === 'undefined'
		? { register: () => {}, unregister: () => {} }
		: new FinalizationRegistry((ptr) => wasm.__wbg_mapsite_free(ptr >>> 0));
/**
 */
export class MapSite {
	static __wrap(ptr) {
		ptr = ptr >>> 0;
		const obj = Object.create(MapSite.prototype);
		obj.__wbg_ptr = ptr;
		MapSiteFinalization.register(obj, obj.__wbg_ptr, obj);
		return obj;
	}

	__destroy_into_raw() {
		const ptr = this.__wbg_ptr;
		this.__wbg_ptr = 0;
		MapSiteFinalization.unregister(this);
		return ptr;
	}

	free() {
		const ptr = this.__destroy_into_raw();
		wasm.__wbg_mapsite_free(ptr);
	}
	/**
	 * @returns {number}
	 */
	get x() {
		const ret = wasm.__wbg_get_mapsite_x(this.__wbg_ptr);
		return ret;
	}
	/**
	 * @param {number} arg0
	 */
	set x(arg0) {
		wasm.__wbg_set_mapsite_x(this.__wbg_ptr, arg0);
	}
	/**
	 * @returns {number}
	 */
	get y() {
		const ret = wasm.__wbg_get_mapsite_y(this.__wbg_ptr);
		return ret;
	}
	/**
	 * @param {number} arg0
	 */
	set y(arg0) {
		wasm.__wbg_set_mapsite_y(this.__wbg_ptr, arg0);
	}
}

const NameFinalization =
	typeof FinalizationRegistry === 'undefined'
		? { register: () => {}, unregister: () => {} }
		: new FinalizationRegistry((ptr) => wasm.__wbg_name_free(ptr >>> 0));
/**
 */
export class Name {
	static __wrap(ptr) {
		ptr = ptr >>> 0;
		const obj = Object.create(Name.prototype);
		obj.__wbg_ptr = ptr;
		NameFinalization.register(obj, obj.__wbg_ptr, obj);
		return obj;
	}

	__destroy_into_raw() {
		const ptr = this.__wbg_ptr;
		this.__wbg_ptr = 0;
		NameFinalization.unregister(this);
		return ptr;
	}

	free() {
		const ptr = this.__destroy_into_raw();
		wasm.__wbg_name_free(ptr);
	}
	/**
	 * @returns {string}
	 */
	name() {
		let deferred1_0;
		let deferred1_1;
		try {
			const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
			wasm.name_name(retptr, this.__wbg_ptr);
			var r0 = getInt32Memory0()[retptr / 4 + 0];
			var r1 = getInt32Memory0()[retptr / 4 + 1];
			deferred1_0 = r0;
			deferred1_1 = r1;
			return getStringFromWasm0(r0, r1);
		} finally {
			wasm.__wbindgen_add_to_stack_pointer(16);
			wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
		}
	}
	/**
	 * @returns {string}
	 */
	reading() {
		let deferred1_0;
		let deferred1_1;
		try {
			const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
			wasm.name_reading(retptr, this.__wbg_ptr);
			var r0 = getInt32Memory0()[retptr / 4 + 0];
			var r1 = getInt32Memory0()[retptr / 4 + 1];
			deferred1_0 = r0;
			deferred1_1 = r1;
			return getStringFromWasm0(r0, r1);
		} finally {
			wasm.__wbindgen_add_to_stack_pointer(16);
			wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
		}
	}
}

const NameSetFinalization =
	typeof FinalizationRegistry === 'undefined'
		? { register: () => {}, unregister: () => {} }
		: new FinalizationRegistry((ptr) => wasm.__wbg_nameset_free(ptr >>> 0));
/**
 */
export class NameSet {
	static __wrap(ptr) {
		ptr = ptr >>> 0;
		const obj = Object.create(NameSet.prototype);
		obj.__wbg_ptr = ptr;
		NameSetFinalization.register(obj, obj.__wbg_ptr, obj);
		return obj;
	}

	__destroy_into_raw() {
		const ptr = this.__wbg_ptr;
		this.__wbg_ptr = 0;
		NameSetFinalization.unregister(this);
		return ptr;
	}

	free() {
		const ptr = this.__destroy_into_raw();
		wasm.__wbg_nameset_free(ptr);
	}
	/**
	 * @returns {Name}
	 */
	city_name() {
		const ret = wasm.nameset_city_name(this.__wbg_ptr);
		return Name.__wrap(ret);
	}
	/**
	 * @returns {Name}
	 */
	county_name() {
		const ret = wasm.nameset_county_name(this.__wbg_ptr);
		return Name.__wrap(ret);
	}
	/**
	 * @returns {Name}
	 */
	subprefecture_name() {
		const ret = wasm.nameset_subprefecture_name(this.__wbg_ptr);
		return Name.__wrap(ret);
	}
	/**
	 * @returns {Name}
	 */
	subprefecture_postfix() {
		const ret = wasm.nameset_subprefecture_postfix(this.__wbg_ptr);
		return Name.__wrap(ret);
	}
	/**
	 * @returns {Name}
	 */
	government() {
		const ret = wasm.nameset_government(this.__wbg_ptr);
		return Name.__wrap(ret);
	}
}

const NetworkNodeFinalization =
	typeof FinalizationRegistry === 'undefined'
		? { register: () => {}, unregister: () => {} }
		: new FinalizationRegistry((ptr) => wasm.__wbg_networknode_free(ptr >>> 0));
/**
 */
export class NetworkNode {
	static __wrap(ptr) {
		ptr = ptr >>> 0;
		const obj = Object.create(NetworkNode.prototype);
		obj.__wbg_ptr = ptr;
		NetworkNodeFinalization.register(obj, obj.__wbg_ptr, obj);
		return obj;
	}

	__destroy_into_raw() {
		const ptr = this.__wbg_ptr;
		this.__wbg_ptr = 0;
		NetworkNodeFinalization.unregister(this);
		return ptr;
	}

	free() {
		const ptr = this.__destroy_into_raw();
		wasm.__wbg_networknode_free(ptr);
	}
	/**
	 * @returns {MapSite}
	 */
	site() {
		const ret = wasm.networknode_site(this.__wbg_ptr);
		return MapSite.__wrap(ret);
	}
}

const NetworkPathFinalization =
	typeof FinalizationRegistry === 'undefined'
		? { register: () => {}, unregister: () => {} }
		: new FinalizationRegistry((ptr) => wasm.__wbg_networkpath_free(ptr >>> 0));
/**
 */
export class NetworkPath {
	static __wrap(ptr) {
		ptr = ptr >>> 0;
		const obj = Object.create(NetworkPath.prototype);
		obj.__wbg_ptr = ptr;
		NetworkPathFinalization.register(obj, obj.__wbg_ptr, obj);
		return obj;
	}

	__destroy_into_raw() {
		const ptr = this.__wbg_ptr;
		this.__wbg_ptr = 0;
		NetworkPathFinalization.unregister(this);
		return ptr;
	}

	free() {
		const ptr = this.__destroy_into_raw();
		wasm.__wbg_networkpath_free(ptr);
	}
	/**
	 * @returns {NetworkNode}
	 */
	node1() {
		const ret = wasm.networkpath_node1(this.__wbg_ptr);
		return NetworkNode.__wrap(ret);
	}
	/**
	 * @returns {NetworkNode}
	 */
	node2() {
		const ret = wasm.networkpath_node2(this.__wbg_ptr);
		return NetworkNode.__wrap(ret);
	}
	/**
	 * @returns {number}
	 */
	stage() {
		const ret = wasm.networkpath_stage(this.__wbg_ptr);
		return ret >>> 0;
	}
}

const StandardMapFinalization =
	typeof FinalizationRegistry === 'undefined'
		? { register: () => {}, unregister: () => {} }
		: new FinalizationRegistry((ptr) => wasm.__wbg_standardmap_free(ptr >>> 0));
/**
 */
export class StandardMap {
	static __wrap(ptr) {
		ptr = ptr >>> 0;
		const obj = Object.create(StandardMap.prototype);
		obj.__wbg_ptr = ptr;
		StandardMapFinalization.register(obj, obj.__wbg_ptr, obj);
		return obj;
	}

	__destroy_into_raw() {
		const ptr = this.__wbg_ptr;
		this.__wbg_ptr = 0;
		StandardMapFinalization.unregister(this);
		return ptr;
	}

	free() {
		const ptr = this.__destroy_into_raw();
		wasm.__wbg_standardmap_free(ptr);
	}
	/**
	 * @param {number} seed
	 * @param {number} x_expand_prop
	 * @returns {StandardMap | undefined}
	 */
	static new(seed, x_expand_prop) {
		const ret = wasm.create_standard_map(seed, x_expand_prop);
		return ret === 0 ? undefined : StandardMap.__wrap(ret);
	}
	/**
	 * @returns {NameSet}
	 */
	get_nameset() {
		const ret = wasm.standardmap_get_nameset(this.__wbg_ptr);
		return NameSet.__wrap(ret);
	}
	/**
	 * @returns {number}
	 */
	get_population() {
		const ret = wasm.standardmap_get_population(this.__wbg_ptr);
		return ret >>> 0;
	}
	/**
	 * @returns {MapSite}
	 */
	bound_min() {
		const ret = wasm.standardmap_bound_min(this.__wbg_ptr);
		return MapSite.__wrap(ret);
	}
	/**
	 * @returns {MapSite}
	 */
	bound_max() {
		const ret = wasm.standardmap_bound_max(this.__wbg_ptr);
		return MapSite.__wrap(ret);
	}
	/**
	 * @param {number} x
	 * @param {number} y
	 * @returns {number | undefined}
	 */
	get_elevation(x, y) {
		try {
			const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
			wasm.standardmap_get_elevation(retptr, this.__wbg_ptr, x, y);
			var r0 = getInt32Memory0()[retptr / 4 + 0];
			var r2 = getFloat64Memory0()[retptr / 8 + 1];
			return r0 === 0 ? undefined : r2;
		} finally {
			wasm.__wbindgen_add_to_stack_pointer(16);
		}
	}
	/**
	 * @returns {(NetworkPath)[]}
	 */
	network_paths() {
		try {
			const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
			wasm.standardmap_network_paths(retptr, this.__wbg_ptr);
			var r0 = getInt32Memory0()[retptr / 4 + 0];
			var r1 = getInt32Memory0()[retptr / 4 + 1];
			var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
			wasm.__wbindgen_free(r0, r1 * 4, 4);
			return v1;
		} finally {
			wasm.__wbindgen_add_to_stack_pointer(16);
		}
	}
	/**
	 * @returns {MapSite}
	 */
	get_origin_site() {
		const ret = wasm.standardmap_get_origin_site(this.__wbg_ptr);
		return MapSite.__wrap(ret);
	}
}

async function __wbg_load(module, imports) {
	if (typeof Response === 'function' && module instanceof Response) {
		if (typeof WebAssembly.instantiateStreaming === 'function') {
			try {
				return await WebAssembly.instantiateStreaming(module, imports);
			} catch (e) {
				if (module.headers.get('Content-Type') != 'application/wasm') {
					console.warn(
						'`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n',
						e
					);
				} else {
					throw e;
				}
			}
		}

		const bytes = await module.arrayBuffer();
		return await WebAssembly.instantiate(bytes, imports);
	} else {
		const instance = await WebAssembly.instantiate(module, imports);

		if (instance instanceof WebAssembly.Instance) {
			return { instance, module };
		} else {
			return instance;
		}
	}
}

function __wbg_get_imports() {
	const imports = {};
	imports.wbg = {};
	imports.wbg.__wbg_networkpath_new = function (arg0) {
		const ret = NetworkPath.__wrap(arg0);
		return addHeapObject(ret);
	};
	imports.wbg.__wbindgen_throw = function (arg0, arg1) {
		throw new Error(getStringFromWasm0(arg0, arg1));
	};

	return imports;
}

function __wbg_init_memory(imports, maybe_memory) {}

function __wbg_finalize_init(instance, module) {
	wasm = instance.exports;
	__wbg_init.__wbindgen_wasm_module = module;
	cachedFloat64Memory0 = null;
	cachedInt32Memory0 = null;
	cachedUint32Memory0 = null;
	cachedUint8Memory0 = null;

	return wasm;
}

function initSync(module) {
	if (wasm !== undefined) return wasm;

	const imports = __wbg_get_imports();

	__wbg_init_memory(imports);

	if (!(module instanceof WebAssembly.Module)) {
		module = new WebAssembly.Module(module);
	}

	const instance = new WebAssembly.Instance(module, imports);

	return __wbg_finalize_init(instance, module);
}

async function __wbg_init(input) {
	if (wasm !== undefined) return wasm;

	if (typeof input === 'undefined') {
		input = new URL('dogen_generator_bg.wasm', import.meta.url);
	}
	const imports = __wbg_get_imports();

	if (
		typeof input === 'string' ||
		(typeof Request === 'function' && input instanceof Request) ||
		(typeof URL === 'function' && input instanceof URL)
	) {
		input = fetch(input);
	}

	__wbg_init_memory(imports);

	const { instance, module } = await __wbg_load(await input, imports);

	return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
