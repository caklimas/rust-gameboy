import { SET_CURRENT_GAME } from "../actions/currentGame";

export function currentGame(currentGame = '', action: any) {
    switch (action.type) {
        case SET_CURRENT_GAME:
            return action.currentGame;
        default:
            return currentGame;
    }
}