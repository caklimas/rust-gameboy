import { ButtonState } from "../redux/state/buttons";
import { DirectionState } from "../redux/state/direction";
import { RustGameboy } from "../redux/state/rustGameboy";

export function getInput(
    rustGameboy: RustGameboy,
    buttons: ButtonState,
    direction: DirectionState
) {
    let input = new rustGameboy.Input!();
    input.a = buttons.a;
    input.b = buttons.b;
    input.start = buttons.start;
    input.select = buttons.select;
    input.up = direction.up;
    input.down = direction.down;
    input.left = direction.left;
    input.right = direction.right;
    return input;
}