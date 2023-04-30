import { GameboyState, defaultState } from '../state/gameboy';
import { LOAD_ROM } from '../actions/gameboy';

export function gameboy(state: GameboyState = defaultState, action: any) {
    switch (action.type) {
        case LOAD_ROM:
            return { ...state, emulator: action.emulator };
        default:
            return state;
    }
}