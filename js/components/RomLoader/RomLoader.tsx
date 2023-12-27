import { useState, useEffect, useCallback } from 'react';
import styled from 'styled-components';
import DropdownButton from 'react-bootstrap/DropdownButton';
import Dropdown from 'react-bootstrap/Dropdown';
import { useDispatch, useSelector } from 'react-redux';
import { useFilePicker } from 'use-file-picker';

import { loadRom } from '../../redux/actions/gameboy';
import { setCurrentGame } from '../../redux/actions/currentGame';
import { loadWasm } from '../../helpers/wasm';
import { State } from '../../redux/state/state';
import { RustGameboy } from '../../redux/state/rustGameboy';
import { Emulator } from 'gameboy';
import { Form } from 'react-bootstrap';

const StyledDropdownButton = styled(DropdownButton)`
  margin-top: 20px;
`;

const StyledCheck = styled(Form.Check)`
  color: white;
`;

export function RomLoader() {
  const dispatch = useDispatch();
  const [gameboy, setGameboy] = useState<RustGameboy | null>(null);
  const [runBootRom, setRunBootRom] = useState(false);
  const [cgb, setCgb] = useState(false);
  const [paused, setPaused] = useState(false);
  const emulator = useSelector<State, Emulator | null>(
    (state) => state.gameboy.emulator
  );
  const [openFileSelector, { plainFiles }] = useFilePicker({
    accept: ['.gb', '.gbc'],
    multiple: false
  });

  useEffect(() => {
    const getGameboy = async () => {
      const gameboy = await loadWasm();
      setGameboy(gameboy);
    };

    getGameboy().catch((error) => console.error(error));
  });

  const readFile = useCallback(
    async (fileName: string) => {
      if (!gameboy?.Emulator || !gameboy?.RomConfig) {
        return;
      }

      const response = await fetch(`/roms/${fileName}`);
      const blob = await response.blob();
      const buffer = await blob.arrayBuffer();
      const bytes = new Uint8Array(buffer);
      const key = fileName.replace(/.gb$/, '');
      const fileData = getFileData(key);
      const romConfig = new gameboy.RomConfig(runBootRom, cgb);
      if (fileData === null) {
        const emulator = new gameboy.Emulator(bytes, romConfig);
        dispatch(setCurrentGame(key));
        dispatch(loadRom(emulator));
      } else {
        const emulator = gameboy.Emulator.from_save_data(
          bytes,
          fileData,
          romConfig
        );
        dispatch(setCurrentGame(key));
        dispatch(loadRom(emulator));
      }
    },
    [runBootRom, cgb, gameboy, dispatch]
  );

  const pickFile = useCallback(
    async (file: File) => {
      if (!gameboy?.Emulator || !gameboy?.RomConfig) {
        return;
      }

      const buffer = await file.arrayBuffer();
      const bytes = new Uint8Array(buffer);
      const romConfig = new gameboy.RomConfig(runBootRom, cgb);
      const emulator = new gameboy.Emulator(bytes, romConfig);
      dispatch(loadRom(emulator));
    },
    [runBootRom, cgb, gameboy, dispatch]
  );

  if (!!emulator) return null;

  if (plainFiles.length > 0) {
    pickFile(plainFiles[0]).catch((error) => console.error(error));
  }

  return (
    <div>
      <StyledDropdownButton
        variant="secondary"
        id="dropdown-basic-button"
        title="Select ROM to play"
      >
        <Dropdown.Item onClick={async () => await readFile('cpu_instrs.gb')}>
          CPU All Tests
        </Dropdown.Item>
        <Dropdown.Item onClick={async () => await readFile('Dr. Mario.gb')}>
          Dr. Mario
        </Dropdown.Item>
        <Dropdown.Item
          onClick={async () => await readFile('Super Mario Land.gb')}
        >
          Super Mario Land
        </Dropdown.Item>
        <Dropdown.Item onClick={async () => await readFile('Tetris.gb')}>
          Tetris
        </Dropdown.Item>
        <Dropdown.Item onClick={async () => await readFile('Tetris DX.gbc')}>
          Tetris DX
        </Dropdown.Item>
        <Dropdown.Item
          onClick={async () => await readFile("Kirby's Dream Land.gb")}
        >
          Kirby&apos;s Dream Land
        </Dropdown.Item>
        <Dropdown.Item onClick={async () => await readFile('Zelda.gb')}>
          The Legend of Zelda Link&apos;s Awakening
        </Dropdown.Item>
        <Dropdown.Item onClick={async () => await readFile('Pokemon Blue.gb')}>
          Pokemon Blue
        </Dropdown.Item>
        <Dropdown.Item onClick={() => openFileSelector()}>
          Pick File...
        </Dropdown.Item>
      </StyledDropdownButton>
      <StyledCheck
        id="run-boot-rom-checkbox"
        type="checkbox"
        checked={runBootRom}
        label="Run Boot ROM"
        onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
          setRunBootRom(e.target.checked)
        }
      />
      <StyledCheck
        id="cgb-checkbox"
        type="checkbox"
        checked={cgb}
        label="Gameboy Color"
        onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
          setCgb(e.target.checked)
        }
      />
    </div>
  );
}

const getFileData = (key: string): Uint8Array | null => {
  if (!window?.localStorage) {
    return null;
  }

  const item = window.localStorage.getItem(key);
  if (item === null) {
    return null;
  }

  return new Uint8Array(JSON.parse(item));
};
