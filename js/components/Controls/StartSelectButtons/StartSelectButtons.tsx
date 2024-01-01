import React, { useCallback } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import styled from 'styled-components';
import ControlButton from '../ControlButton/ControlButton';
import { ButtonState } from '../../../redux/state/buttons';
import { State } from '../../../redux/state/state';
import { getInput } from '../../../helpers/input';
import { RustGameboy } from '../../../redux/state/rustGameboy';
import { setButtons } from '../../../redux/actions/buttons';
import { DirectionState } from '../../../redux/state/direction';
import { mediaMinMd } from '../../../constants/screenSizes';
import GridCell from '../../GridCell/GridCell';
import { Emulator } from 'gameboy';
import { AllButtons } from '../AllButtons/AllButtons';

interface Props {
  isMobile?: boolean;
}

interface StateProps {
  buttons: ButtonState;
  direction: DirectionState;
  emulator: Emulator;
  rustGameboy: RustGameboy;
}

type ButtonKey = 'start' | 'select';

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

export function StartSelectButtons({ isMobile }: Props) {
  const dispatch = useDispatch();
  const stateProps = useSelector<State, StateProps>((state) => ({
    buttons: state.buttons,
    direction: state.direction,
    emulator: state.gameboy.emulator!,
    rustGameboy: state.rustGameboy
  }));

  const handleTouch = useCallback(
    (
      e: React.TouchEvent<HTMLElement>,
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
    <StyledStartSelectControls>
      <GridCell column={1} row={1}>
        <ControlButton
          pressed={stateProps.buttons.start}
          text="Start"
          type="start-select"
          onTouchStart={(e) => handleTouch(e, 'start', true)}
          onTouchEnd={(e) => handleTouch(e, 'start', false)}
          onTouchCancel={(e) => handleTouch(e, 'start', false)}
        />
      </GridCell>
      <GridCell column={2} row={1}>
        <ControlButton
          pressed={stateProps.buttons.select}
          text="Select"
          type="start-select"
          onTouchStart={(e) => handleTouch(e, 'select', true)}
          onTouchEnd={(e) => handleTouch(e, 'select', false)}
          onTouchCancel={(e) => handleTouch(e, 'select', false)}
        />
      </GridCell>
      {isMobile && (
        <GridCell column={3} row={1}>
          <AllButtons />
        </GridCell>
      )}
    </StyledStartSelectControls>
  );
}
