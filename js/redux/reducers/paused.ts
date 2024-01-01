import { SET_PAUSED } from "../actions/paused";

export function paused(paused = false, action: any) {
    switch (action.type) {
        case SET_PAUSED:
            return action.paused;
        default:
            return paused;
    }
}