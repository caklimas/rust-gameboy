import { Emulator } from "caklimas-rust-gameboy";

export interface GameboyState {
    emulator: Emulator | null
};

export const defaultState: GameboyState = {
    emulator: null
};