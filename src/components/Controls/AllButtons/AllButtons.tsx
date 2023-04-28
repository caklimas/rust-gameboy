import { Emulator } from "caklimas-rust-gameboy";
import { connect } from "react-redux";
import { getInput } from "../../../helpers/input";
import { setButtons } from "../../../redux/actions/buttons";
import { ButtonState } from "../../../redux/state/buttons";
import { DirectionState } from "../../../redux/state/direction";
import { RustGameboy } from "../../../redux/state/rustGameboy";
import { State } from "../../../redux/state/state";
import ControlButton from "../ControlButton/ControlButton";

type Props = StateProps & DispatchProps;

interface StateProps {
    buttons: ButtonState;
    direction: DirectionState;
    emulator: Emulator;
    rustGameboy: RustGameboy;
}

interface DispatchProps {
    setButtons(buttons: ButtonState): void;
}

const mapStateToProps = (state: State): StateProps => ({
    buttons: state.buttons,
    direction: state.direction,
    emulator: state.gameboy.emulator!,
    rustGameboy: state.rustGameboy,
});

const mapDispatchToProps = (dispatch: any): DispatchProps => ({
    setButtons: (buttons: ButtonState) => dispatch(setButtons(buttons)),
});

const AllButtons = (props: Props) => {
    return (<ControlButton
        pressed={false}
        text="All"
        type="start-select"
        onTouchStart={(e) => handleTouch(e, props, true)}
        onTouchEnd={(e) => handleTouch(e, props, false)}
        onTouchCancel={(e) => handleTouch(e, props, false)}
    />);
}

const handleTouch = (
    e: React.TouchEvent<HTMLElement>,
    props: Props,
    pressed: boolean
) => {
    const updatedState = { ...props.buttons, a: pressed, b: pressed, start: pressed, select: pressed };
    const input = getInput(props.rustGameboy, updatedState, props.direction);
    props.setButtons(updatedState);
    props.emulator.update_controls(input);

    if (pressed) {
        window.navigator.vibrate(10);
    }
};

export default connect(mapStateToProps, mapDispatchToProps)(AllButtons);