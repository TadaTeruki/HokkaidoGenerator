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

const NameFinalization =
	typeof FinalizationRegistry === 'undefined'
		? { register: () => {}, unregister: () => {} }
		: new FinalizationRegistry((ptr) => wasm.__wbg_name_free(ptr >>> 0));
/**
 */
export class Name {
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
}

const NameSetFinalization =
	typeof FinalizationRegistry === 'undefined'
		? { register: () => {}, unregister: () => {} }
		: new FinalizationRegistry((ptr) => wasm.__wbg_nameset_free(ptr >>> 0));
/**
 */
export class NameSet {
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
}

const NetworkNodeFinalization =
	typeof FinalizationRegistry === 'undefined'
		? { register: () => {}, unregister: () => {} }
		: new FinalizationRegistry((ptr) => wasm.__wbg_networknode_free(ptr >>> 0));
/**
 */
export class NetworkNode {
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
	 * @returns {number}
	 */
	x() {
		const ret = wasm.networknode_x(this.__wbg_ptr);
		return ret;
	}
	/**
	 * @returns {number}
	 */
	y() {
		const ret = wasm.networknode_y(this.__wbg_ptr);
		return ret;
	}
	/**
	 * @returns {number}
	 */
	stage() {
		const ret = wasm.networknode_stage(this.__wbg_ptr);
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
	imports.wbg.__wbindgen_throw = function (arg0, arg1) {
		throw new Error(getStringFromWasm0(arg0, arg1));
	};

	return imports;
}

function __wbg_init_memory(imports, maybe_memory) {}

function __wbg_finalize_init(instance, module) {
	wasm = instance.exports;
	__wbg_init.__wbindgen_wasm_module = module;
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
