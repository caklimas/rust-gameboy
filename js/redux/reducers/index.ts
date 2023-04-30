import { combineReducers } from 'redux';
import { buttons } from './buttons';
import { currentGame } from './currentGame';
import { direction } from './direction';
import { gameboy } from './gameboy';
import { rustGameboy } from './rustGameboy';

export const rootReducer = combineReducers({
    buttons,
    currentGame,
    direction,
    gameboy,
    rustGameboy
});