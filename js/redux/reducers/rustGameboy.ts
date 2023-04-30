import { SET_RUST_GAMEBOY } from '../actions/rustGameboy';
import { RustGameboy, defaultState } from '../state/rustGameboy';

export function rustGameboy(state: RustGameboy = defaultState, action: any) {
    switch(action.type) {
        case SET_RUST_GAMEBOY:
            return { ...state, ...action.rustGameboy };
        default:
            return state;
    }
}