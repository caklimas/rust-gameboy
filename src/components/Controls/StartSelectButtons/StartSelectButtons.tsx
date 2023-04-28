import React from "react";
import { connect } from "react-redux";
import styled from "styled-components";
import ControlButton from "../ControlButton/ControlButton";
import { ButtonState } from "../../../redux/state/buttons";
import { State } from "../../../redux/state/state";
import { getInput } from "../../../helpers/input";
import { RustGameboy } from "../../../redux/state/rustGameboy";
import { setButtons } from "../../../redux/actions/buttons";
import { DirectionState } from "../../../redux/state/direction";
import { mediaMinMd } from "../../../constants/screenSizes";
import GridCell from "../../GridCell/GridCell";
import { Emulator } from "caklimas-rust-gameboy";
import AllButtons from "../AllButtons/AllButtons";

type Props = OwnProps & StateProps & DispatchProps;

interface OwnProps {
  isMobile?: boolean;
};

interface StateProps {
  buttons: ButtonState;
  direction: DirectionState;
  emulator: Emulator;
  rustGameboy: RustGameboy;
}

interface DispatchProps {
  setButtons(buttons: ButtonState): void;
}

type ButtonKey = "start" | "select";

const StyledStartSelectControls = styled.div`
  display: grid;
  gap: 10px;
  grid-template-columns: repeat(3, 75px);
  justify-content: center;
  margin-top: -55px;

  @media only screen and (min-width: ${mediaMinMd}px) {
    grid-template-columns: repeat(2, 75px);
    margin-top: 35px;
  }
`;

const StartSelectButtons = (props: Props) => {
  return (
    <StyledStartSelectControls>
      <GridCell column={1} row={1}>
        <ControlButton
          pressed={props.buttons.start}
          text="Start"
          type="start-select"
          onTouchStart={(e) => handleTouch(e, props, "start", true)}
          onTouchEnd={(e) => handleTouch(e, props, "start", false)}
          onTouchCancel={(e) => handleTouch(e, props, "start", false)}
        />
      </GridCell>
      <GridCell column={2} row={1}>
        <ControlButton
          pressed={props.buttons.select}
          text="Select"
          type="start-select"
          onTouchStart={(e) => handleTouch(e, props, "select", true)}
          onTouchEnd={(e) => handleTouch(e, props, "select", false)}
          onTouchCancel={(e) => handleTouch(e, props, "select", false)}
        />
      </GridCell>
      {
        props.isMobile && (
          <GridCell column={3} row={1}>
            <AllButtons />
          </GridCell>
        )
      }
    </StyledStartSelectControls>
  );
};

const handleTouch = (
  e: React.TouchEvent<HTMLElement>,
  props: Props,
  buttonKey: ButtonKey,
  pressed: boolean
) => {
  const updatedState = { ...props.buttons, [buttonKey]: pressed };
  const input = getInput(props.rustGameboy, updatedState, props.direction);
  props.setButtons(updatedState);
  props.emulator.update_controls(input);

  if (pressed) {
    window.navigator.vibrate(10);
  }
};

const mapStateToProps = (state: State): StateProps => ({
  buttons: state.buttons,
  direction: state.direction,
  emulator: state.gameboy.emulator!,
  rustGameboy: state.rustGameboy,
});

const mapDispatchToProps = (dispatch: any): DispatchProps => ({
  setButtons: (buttons: ButtonState) => dispatch(setButtons(buttons)),
});

export default connect(mapStateToProps, mapDispatchToProps)(StartSelectButtons);
