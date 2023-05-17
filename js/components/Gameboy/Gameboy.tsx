import { useSelector } from 'react-redux';
import styled from 'styled-components';
import { useMediaQuery } from 'react-responsive';
import { useBeforeunload } from 'react-beforeunload';
import { Button, Form, ToggleButton } from 'react-bootstrap';
import { useCallback, useState } from 'react';
import gameboyDimensions from '../../constants/gameboy';
import { mobileMediaQuery } from '../../helpers/mediaQueries';
import { Screen } from '../Screen/Screen';
import Controls from '../Controls/Controls';
import { mediaMinMd } from '../../constants/screenSizes';
import { Emulator } from 'gameboy';
import { State } from '../../redux/state/state';
import { EmulatorInfo } from '../EmulatorInfo/EmulatorInfo';

const StyledGameboy = styled.div`
  background-color: #bababa;
  border-top-left-radius: 10px;
  border-top-right-radius: 10px;
  border-bottom-left-radius: 5px;
  border-bottom-right-radius: 25px;
  display: flex;
  flex-direction: column;
  height: 500px;
  justify-content: center;
  margin-top: 20px;
  width: 350px;

  @media only screen and (min-width: ${mediaMinMd}px) {
    height: 700px;
    width: 540px;
  }
`;

const GameboyContainer = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
`;

const ShowInfoButton = styled(Button)`
  margin-top: 10px;
  height: 50px;
  width: 100px;
`;

const ToggleColor = styled(Form.Check)`
  left: 76%;
`;

export function Gameboy() {
  useBeforeunload(() => {
    if (emulator && currentGame) {
      saveToLocalStorage(currentGame, emulator.save());
    }
  });

  const [checked, setChecked] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const handleOpen = useCallback(() => setShowModal(true), [setShowModal]);
  const isMobile = useMediaQuery(mobileMediaQuery);
  const pixelSize = isMobile ? 1 : 3;
  const currentGame = useSelector<State, string>((state) => state.currentGame);
  const emulator = useSelector<State, Emulator>(
    (state) => state.gameboy.emulator!
  );
  if (!emulator) return null;

  const toggleColor = (value: boolean) => {
    emulator.toggle_color(value);
    setChecked(value);
  };

  return (
    <GameboyContainer>
      <StyledGameboy>
        <ToggleColor
          id="color-checkbox"
          type="checkbox"
          checked={checked}
          label="Toggle color"
          onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
            toggleColor(e.target.checked)
          }
        />
        <Screen
          width={gameboyDimensions.width}
          height={gameboyDimensions.height}
          pixelSize={pixelSize}
        />
        <Controls />
      </StyledGameboy>
      {!isMobile && (
        <ShowInfoButton onClick={handleOpen}>Show Info</ShowInfoButton>
      )}
      {!isMobile && showModal && (
        <EmulatorInfo show={showModal} setShow={setShowModal} />
      )}
    </GameboyContainer>
  );
}

const saveToLocalStorage = (key: string, data: Uint8Array) => {
  if (!window || !key) {
    return;
  }

  window.localStorage.setItem(key, JSON.stringify(Array.from(data)));
};
