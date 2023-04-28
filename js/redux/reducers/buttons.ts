import { SET_BUTTONS } from '../actions/buttons';
import { ButtonState, defaultState } from '../state/buttons';

export function buttons(state: ButtonState = defaultState, action: any) {
    switch(action.type) {
        case SET_BUTTONS:
            return { ...state, ...action.buttons };
        default:
            return state;
    }
}