import { Emulator } from "../../../pkg";

export const LOAD_ROM = Symbol("LOAD_ROM");

export const loadRom = (emulator: Emulator) => ({
  type: LOAD_ROM,
  emulator,
});
