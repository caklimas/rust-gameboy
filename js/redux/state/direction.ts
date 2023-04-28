export interface DirectionState {
    up: boolean,
    down: boolean,
    left: boolean,
    right: boolean
}

export const defaultState: DirectionState = {
    up: false,
    down: false,
    left: false,
    right: false
}