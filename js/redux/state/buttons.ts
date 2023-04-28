export interface ButtonState {
    a: boolean,
    b: boolean,
    start: boolean,
    select: boolean
}

export const defaultState: ButtonState = {
    a: false,
    b: false,
    start: false,
    select: false
};