import { Emulator } from 'gameboy';
import { useDispatch, useSelector } from 'react-redux';
import { getInput } from '../../../helpers/input';
import { setButtons } from '../../../redux/actions/buttons';
import { ButtonState } from '../../../redux/state/buttons';
import { DirectionState } from '../../../redux/state/direction';
import { RustGameboy } from '../../../redux/state/rustGameboy';
import { State } from '../../../redux/state/state';
import ControlButton from '../ControlButton/ControlButton';
import { useCallback } from 'react';

interface StateProps {
  buttons: ButtonState;
  direction: DirectionState;
  emulator: Emulator;
  rustGameboy: RustGameboy;
}

export function AllButtons() {
  const dispatch = useDispatch();
  const stateProps = useSelector<State, StateProps>((state) => ({
    buttons: state.buttons,
    direction: state.direction,
    emulator: state.gameboy.emulator!,
    rustGameboy: state.rustGameboy
  }));

  const handleTouch = useCallback(
    (e: React.TouchEvent<HTMLElement>, pressed: boolean) => {
      const updatedState = {
        ...stateProps.buttons,
        a: pressed,
        b: pressed,
        start: pressed,
        select: pressed
      };
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
    <ControlButton
      pressed={false}
      text="All"
      type="start-select"
      onTouchStart={(e) => handleTouch(e, true)}
      onTouchEnd={(e) => handleTouch(e, false)}
      onTouchCancel={(e) => handleTouch(e, false)}
    />
  );
}
