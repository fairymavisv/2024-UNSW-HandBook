/* tslint:disable */
/* eslint-disable */
/**
* @param {string} input
* @returns {string}
*/
export function test_io(input: string): string;
/**
* @param {number} num_threads
* @returns {Promise<any>}
*/
export function initThreadPool(num_threads: number): Promise<any>;
/**
* @param {number} receiver
*/
export function wbg_rayon_start_worker(receiver: number): void;
export interface JsCourseInfo {
    code: string;
    name: string;
    uoc: number;
    description: string;
    conditions: string;
    offerings: string[];
}

export interface JsProgramInfo {
    name: string;
    code: string;
    uoc: string;
    overview: string;
    structure_summary: string;
    structure: JsProgramStructure;
}

export interface JsProgramStructure {
    course_list: [string, string[]][];
    specialisation_list: [string, string[]][];
}

/**
*/
export class HandbookDataInterface {
  free(): void;
/**
* @param {string} data_src_path
* @returns {HandbookDataInterface}
*/
  static new(data_src_path: string): HandbookDataInterface;
/**
* @param {string} code
* @returns {JsCourseInfo | undefined}
*/
  get_course_info(code: string): JsCourseInfo | undefined;
/**
* @param {string} code
* @returns {JsProgramInfo | undefined}
*/
  get_program_info(code: string): JsProgramInfo | undefined;
/**
* @param {string} code
* @param {(string)[] | undefined} [spec]
* @returns {JsProgramInfo | undefined}
*/
  get_program_and_spec_info(code: string, spec?: (string)[]): JsProgramInfo | undefined;
/**
* @param {string} program_code
* @returns {(string)[] | undefined}
*/
  list_program_all_coursecodes(program_code: string): (string)[] | undefined;
/**
* @param {string} program_code
* @param {(string)[]} taken_course
* @param {number | undefined} [wam]
* @returns {(string)[] | undefined}
*/
  list_eligable_courses(program_code: string, taken_course: (string)[], wam?: number): (string)[] | undefined;
}
/**
*/
export class JsSpecialisationInfo {
  free(): void;
}
/**
*/
export class wbg_rayon_PoolBuilder {
  free(): void;
/**
* @returns {number}
*/
  numThreads(): number;
/**
* @returns {number}
*/
  receiver(): number;
/**
*/
  build(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly test_io: (a: number, b: number, c: number) => void;
  readonly __wbg_handbookdatainterface_free: (a: number) => void;
  readonly __wbg_jsspecialisationinfo_free: (a: number) => void;
  readonly handbookdatainterface_new: (a: number, b: number) => number;
  readonly handbookdatainterface_get_course_info: (a: number, b: number, c: number) => number;
  readonly handbookdatainterface_get_program_info: (a: number, b: number, c: number) => number;
  readonly handbookdatainterface_get_program_and_spec_info: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly handbookdatainterface_list_program_all_coursecodes: (a: number, b: number, c: number, d: number) => void;
  readonly handbookdatainterface_list_eligable_courses: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly __wbg_wbg_rayon_poolbuilder_free: (a: number) => void;
  readonly wbg_rayon_poolbuilder_numThreads: (a: number) => number;
  readonly wbg_rayon_poolbuilder_receiver: (a: number) => number;
  readonly wbg_rayon_poolbuilder_build: (a: number) => void;
  readonly initThreadPool: (a: number) => number;
  readonly wbg_rayon_start_worker: (a: number) => void;
  readonly memory: WebAssembly.Memory;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_thread_destroy: (a?: number, b?: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
* @param {WebAssembly.Memory} maybe_memory
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput, maybe_memory?: WebAssembly.Memory): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
* @param {WebAssembly.Memory} maybe_memory
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>, maybe_memory?: WebAssembly.Memory): Promise<InitOutput>;
