import { Emulator } from 'gameboy';
import { useSelector, useDispatch } from 'react-redux';
import { useCallback } from 'react';
import styled from 'styled-components';
import { getInput } from '../../../helpers/input';
import { State } from '../../../redux/state/state';
import { ButtonState } from '../../../redux/state/buttons';
import { setButtons } from '../../../redux/actions/buttons';
import { DirectionState } from '../../../redux/state/direction';
import { RustGameboy } from '../../../redux/state/rustGameboy';
import ControlButton from '../ControlButton/ControlButton';
import { mediaMinMd } from '../../../constants/screenSizes';
import GridCell from '../../GridCell/GridCell';

interface StateProps {
  buttons: ButtonState;
  direction: DirectionState;
  emulator: Emulator;
  rustGameboy: RustGameboy;
}

type ButtonKey = 'a' | 'b';

const StyledAbControls = styled.div`
  bottom: 90px;
  display: inline-grid;
  grid-template-columns: repeat(2, 50px);
  left: 75px;
  position: relative;

  @media only screen and (min-width: ${mediaMinMd}px) {
    bottom: 0;
    position: static;
    left: 0;
  }
`;

export function AbButtons() {
  const dispatch = useDispatch();
  const stateProps = useSelector<State, StateProps>((state) => ({
    buttons: state.buttons,
    direction: state.direction,
    emulator: state.gameboy.emulator!,
    rustGameboy: state.rustGameboy
  }));

  const handleTouch = useCallback(
    (
      _e: React.TouchEvent<HTMLElement>,
      buttonKey: ButtonKey,
      pressed: boolean
    ) => {
      const updatedState = { ...stateProps.buttons, [buttonKey]: pressed };
      const input = getInput(
        stateProps.rustGameboy,
        updatedState,
        stateProps.direction
      );
      dispatch(setButtons(updatedState));
      stateProps.emulator.update_controls(input);

      if (pressed) {
        window.navigator.vibrate(10);
      }
    },
    [
      dispatch,
      stateProps.buttons,
      stateProps.direction,
      stateProps.emulator,
      stateProps.rustGameboy
    ]
  );

  return (
    <StyledAbControls>
      <GridCell column={2} row={1}>
        <ControlButton
          pressed={stateProps.buttons.a}
          text="A"
          type="circle"
          onTouchStart={(e) => handleTouch(e, 'a', true)}
          onTouchEnd={(e) => handleTouch(e, 'a', false)}
          onTouchCancel={(e) => handleTouch(e, 'a', false)}
        />
      </GridCell>
      <GridCell column={1} row={2}>
        <ControlButton
          pressed={stateProps.buttons.b}
          text="B"
          type="circle"
          onTouchStart={(e) => handleTouch(e, 'b', true)}
          onTouchEnd={(e) => handleTouch(e, 'b', false)}
          onTouchCancel={(e) => handleTouch(e, 'b', false)}
        />
      </GridCell>
    </StyledAbControls>
  );
}
