import * as wasm from "./loader_bg.wasm";
import { __wbg_set_wasm } from "./loader_bg.js";
__wbg_set_wasm(wasm);
export * from "./loader_bg.js";
