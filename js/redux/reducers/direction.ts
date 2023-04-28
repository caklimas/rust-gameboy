import { DirectionState, defaultState } from '../state/direction';
import {
    SET_DIRECTION,
    CLEAR_DIRECTION
} from '../actions/direction';

export function direction(state: DirectionState = defaultState, action: any): DirectionState {
    switch (action.type) {
        case SET_DIRECTION:
            return { ...state, ...action.direction };
        case CLEAR_DIRECTION:
            return { ...state, ...defaultState };
        default:
            return state;
    }
};