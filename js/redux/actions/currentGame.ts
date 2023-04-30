export const SET_CURRENT_GAME = Symbol("SET_CURRENT_GAME");

export const setCurrentGame = (currentGame: string) => ({
    type: SET_CURRENT_GAME,
    currentGame
});