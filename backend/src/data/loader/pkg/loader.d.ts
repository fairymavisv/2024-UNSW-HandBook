/* tslint:disable */
/* eslint-disable */
/**
* @param {string} input
* @returns {string}
*/
export function test_io(input: string): string;
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
}
/**
*/
export class JsSpecialisationInfo {
  free(): void;
}
