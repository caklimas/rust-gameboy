import { ButtonState } from '../state/buttons';

export const SET_BUTTONS = Symbol('SET_BUTTONS');

export const setButtons = (buttons: ButtonState) => ({
    type: SET_BUTTONS,
    buttons
});