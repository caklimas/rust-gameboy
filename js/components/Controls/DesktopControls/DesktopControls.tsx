import { connect } from "react-redux";
import styled from "styled-components";
// @ts-ignore
import KeyboardEventHandler from "react-keyboard-event-handler";
import { getInput } from "../../../helpers/input";
import { State } from "../../../redux/state/state";
import { ButtonState } from "../../../redux/state/buttons";
import { DirectionState } from "../../../redux/state/direction";
import { setButtons } from "../../../redux/actions/buttons";
import { setDirection } from "../../../redux/actions/direction";
import { RustGameboy } from "../../../redux/state/rustGameboy";
import AbButtons from "../AbButtons/AbButtons";
import ControlButton from "../ControlButton/ControlButton";
import StartSelectButtons from "../StartSelectButtons/StartSelectButtons";
import GridCell from "../../GridCell/GridCell";
import { Emulator } from "../../../../pkg";

const handleKeys = ["up", "down", "left", "right", "z", "x", "shift", "enter"];
const keyMapping = new Map();
keyMapping.set("x", "a");
keyMapping.set("z", "b");
keyMapping.set("enter", "start");
keyMapping.set("shift", "select");

type Props = StateProps & DispatchProps;

interface StateProps {
  buttons: ButtonState;
  direction: DirectionState;
  emulator: Emulator;
  rustGameboy: RustGameboy;
}

interface DispatchProps {
  setButtons(buttons: ButtonState): void;
  setDirection(direction: DirectionState): void;
}

const StyledDesktopControls = styled.div`
  margin: 20px 20px 0;
`;

const StyledDirectionalControls = styled.div`
  display: inline-grid;
  grid-template-columns: repeat(3, 60px);
  margin-right: 195px;
`;

const DesktopControls = (props: Props) => (
  <StyledDesktopControls>
    {renderUpperControls(props.direction)}
    <StartSelectButtons />
    {renderKeyboardHandlers(props)}
  </StyledDesktopControls>
);

const renderUpperControls = (directionState: DirectionState) => {
  return (
    <>
      <StyledDirectionalControls>
        <GridCell column={2} row={1}>
          <ControlButton
            pressed={directionState.up}
            text="↑"
            type="directional"
          />
        </GridCell>
        <GridCell column={2} row={3}>
          <ControlButton
            pressed={directionState.down}
            text="↓"
            type="directional"
          />
        </GridCell>
        <GridCell column={1} row={2}>
          <ControlButton
            pressed={directionState.left}
            text="←"
            type="directional"
          />
        </GridCell>
        <GridCell column={3} row={2}>
          <ControlButton
            pressed={directionState.right}
            text="→"
            type="directional"
          />
        </GridCell>
      </StyledDirectionalControls>
      <AbButtons />
    </>
  );
};

const renderKeyboardHandlers = (props: Props) => {
  return (
    <>
      <KeyboardEventHandler
        handleKeys={handleKeys}
        handleEventType="keydown"
        onKeyEvent={(key: string, _e: any) => {
          if (keyMapping.has(key)) {
            let p: keyof ButtonState = keyMapping.get(key) as any;
            if (props.buttons[p]) return;

            const updatedButtons = { ...props.buttons, [p]: true };
            const input = getInput(
              props.rustGameboy,
              updatedButtons,
              props.direction
            );
            props.setButtons(updatedButtons);
            props.emulator.update_controls(input);
          } else {
            const p: keyof DirectionState = key as any;
            if (props.direction[p]) return;

            const updatedDirection = { ...props.direction, [p]: true };
            const input = getInput(
              props.rustGameboy,
              props.buttons,
              updatedDirection
            );
            props.setDirection(updatedDirection);
            props.emulator.update_controls(input);
          }
        }}
      />
      <KeyboardEventHandler
        handleKeys={handleKeys}
        handleEventType="keyup"
        onKeyEvent={(key: any, _e: any) => {
          if (keyMapping.has(key)) {
            let p: keyof ButtonState = keyMapping.get(key) as any;
            if (!props.buttons[p]) return;

            const updatedButtons = { ...props.buttons, [p]: false };
            const input = getInput(
              props.rustGameboy,
              updatedButtons,
              props.direction
            );
            props.setButtons(updatedButtons);
            props.emulator.update_controls(input);
          } else {
            let p: keyof DirectionState = key as any;
            if (!props.direction[p]) return;

            const updatedDirection = { ...props.direction, [p]: false };
            const input = getInput(
              props.rustGameboy,
              props.buttons,
              updatedDirection
            );
            props.setDirection(updatedDirection);
            props.emulator.update_controls(input);
          }
        }}
      />
    </>
  );
};

const mapStateToProps = (state: State): StateProps => ({
  buttons: state.buttons,
  direction: state.direction,
  emulator: state.gameboy.emulator!,
  rustGameboy: state.rustGameboy,
});

const mapDispatchToProps = (dispatch: any): DispatchProps => ({
  setButtons: (buttons: ButtonState) => dispatch(setButtons(buttons)),
  setDirection: (direction: DirectionState) =>
    dispatch(setDirection(direction)),
});

export default connect(mapStateToProps, mapDispatchToProps)(DesktopControls);
