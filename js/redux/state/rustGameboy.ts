import { Emulator, EmulatorState, Input } from "gameboy";

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
