import { Emulator, EmulatorState, Input, RomConfig } from 'gameboy';

export interface RustGameboy {
  Input: typeof Input | null;
  Emulator: typeof Emulator | null;
  EmulatorState: typeof EmulatorState | null;
  RomConfig: typeof RomConfig | null;
}

export const defaultState: RustGameboy = {
  Input: null,
  Emulator: null,
  EmulatorState: null,
  RomConfig: null
};
