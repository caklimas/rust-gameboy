import { RustGameboy } from "../state/rustGameboy";

export const SET_RUST_GAMEBOY = Symbol('SET_RUST_GAMEBOY');

export const setRustGameboy = (rustGameboy: RustGameboy) => ({
    type: SET_RUST_GAMEBOY,
    rustGameboy
});