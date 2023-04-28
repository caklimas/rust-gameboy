import { DirectionState } from "../redux/state/direction";

const upAngle = 90;
const downAngle = 270;
const leftAngle = 180;
const rightAngle = 0;
const offset = 60;

export function getDirectionFromAngle(angle: number): DirectionState {
    return {
        up: angle >= upAngle - offset && angle <= upAngle + offset,
        down: angle >= downAngle - offset && angle <= downAngle + offset,
        left: angle >= leftAngle - offset && angle <= leftAngle + offset,
        right: angle >= ((rightAngle - offset) + 360) || angle <= rightAngle + offset
    };
}