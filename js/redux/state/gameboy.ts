import { Emulator } from "../../../pkg";

export interface GameboyState {
  emulator: Emulator | null;
}

export const defaultState: GameboyState = {
  emulator: null,
};
