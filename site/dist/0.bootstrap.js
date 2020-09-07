(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/rust_gameboy.js":
/*!******************************!*\
  !*** ../pkg/rust_gameboy.js ***!
  \******************************/
/*! exports provided: run */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _rust_gameboy_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./rust_gameboy_bg.wasm */ \"../pkg/rust_gameboy_bg.wasm\");\n/* harmony import */ var _rust_gameboy_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./rust_gameboy_bg.js */ \"../pkg/rust_gameboy_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"run\", function() { return _rust_gameboy_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"run\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/rust_gameboy.js?");

/***/ }),

/***/ "../pkg/rust_gameboy_bg.js":
/*!*********************************!*\
  !*** ../pkg/rust_gameboy_bg.js ***!
  \*********************************/
/*! exports provided: run */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"run\", function() { return run; });\n/* harmony import */ var _rust_gameboy_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./rust_gameboy_bg.wasm */ \"../pkg/rust_gameboy_bg.wasm\");\n\n\n/**\n* @returns {number}\n*/\nfunction run() {\n    var ret = _rust_gameboy_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"run\"]();\n    return ret;\n}\n\n\n\n//# sourceURL=webpack:///../pkg/rust_gameboy_bg.js?");

/***/ }),

/***/ "../pkg/rust_gameboy_bg.wasm":
/*!***********************************!*\
  !*** ../pkg/rust_gameboy_bg.wasm ***!
  \***********************************/
/*! exports provided: memory, run */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/rust_gameboy_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var rust_gameboy__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! rust-gameboy */ \"../pkg/rust_gameboy.js\");\n\r\n\r\nlet result = rust_gameboy__WEBPACK_IMPORTED_MODULE_0__[\"run\"]();\r\nconsole.log(result);\n\n//# sourceURL=webpack:///./index.js?");

/***/ })

}]);