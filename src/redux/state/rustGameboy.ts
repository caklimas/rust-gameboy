import { Emulator, EmulatorState, Input } from "caklimas-rust-gameboy";

export interface RustGameboy {
  Input: typeof Input | null;
  Emulator: typeof Emulator | null;
  EmulatorState: typeof EmulatorState | null;
}

export const defaultState: RustGameboy = {
  Input: null,
  Emulator: null,
  EmulatorState: null,
};
