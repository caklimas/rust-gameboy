import { ButtonState, defaultState as buttonState } from './buttons';
import { DirectionState, defaultState as directionState } from './direction';
import { GameboyState, defaultState as gameboyState } from './gameboy';
import { RustGameboy, defaultState as rustGameboyState } from './rustGameboy';

export interface State {
    buttons: ButtonState,
    direction: DirectionState,
    gameboy: GameboyState,
    rustGameboy: RustGameboy,
    currentGame: string
};

export const defaultState: State = {
    buttons: buttonState,
    direction: directionState,
    gameboy: gameboyState,
    rustGameboy: rustGameboyState,
    currentGame: ''
};