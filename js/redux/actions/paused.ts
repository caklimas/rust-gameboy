export const SET_PAUSED = Symbol("SET_PAUSED");

export const setPaused = (paused: boolean) => ({
    type: SET_PAUSED,
    paused
});