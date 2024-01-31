import {HandbookDataInterface} from "./pkg";
import {initThreadPool} from "./pkg/loader_bg.wasm";
// import { JsCourseInfo, JsProgramInfo, JsProgramStructure, JsSpecialisationInfo } from "./pkg";

initThreadPool(navigator.hardwareConcurrency);
export const handbook_interface = HandbookDataInterface.new("/root/UNSW-HandBookX/backend/data/");

// Please refers the header files in "./pkg/loader.d.ts"

