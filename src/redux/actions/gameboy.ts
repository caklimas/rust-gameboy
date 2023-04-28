import { Emulator } from "caklimas-rust-gameboy";

export const LOAD_ROM = Symbol('LOAD_ROM');

export const loadRom = (emulator: Emulator) => ({
    type: LOAD_ROM,
    emulator
});