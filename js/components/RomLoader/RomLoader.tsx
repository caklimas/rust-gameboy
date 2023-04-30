import { useState, useEffect } from "react";
import styled from "styled-components";
import DropdownButton from "react-bootstrap/DropdownButton";
import Dropdown from "react-bootstrap/Dropdown";
import { connect } from "react-redux";
import { useFilePicker } from "use-file-picker";

import { loadRom } from "../../redux/actions/gameboy";
import { setCurrentGame } from "../../redux/actions/currentGame";
import { loadWasm } from "../../helpers/wasm";
import { State } from "../../redux/state/state";
import { RustGameboy } from "../../redux/state/rustGameboy";
import { Emulator } from "gameboy";

type Props = StateProps & DispatchProps;

interface StateProps {
  emulator: Emulator | null;
}

interface DispatchProps {
  loadRom: (emulator: Emulator) => any;
  setCurrentGame: (currentGame: string) => any;
}

const StyledDropdownButton = styled(DropdownButton)`
  margin-top: 20px;
`;

const RomLoader = (props: Props) => {
  const [gameboy, setGameboy] = useState<RustGameboy | null>(null);
  const [openFileSelector, { plainFiles }] = useFilePicker({
    accept: ".gb",
    multiple: false,
  });

  useEffect(() => {
    const getGameboy = async () => {
      const gameboy = await loadWasm();
      setGameboy(gameboy);
    };

    getGameboy();
  });

  if (!!props.emulator) return null;

  if (plainFiles.length > 0) {
    pickFile(props, gameboy, plainFiles[0]);
  }

  return (
    <StyledDropdownButton
      variant="secondary"
      id="dropdown-basic-button"
      title="Select ROM to play"
    >
      <Dropdown.Item
        onClick={async () => await readFile(props, gameboy, "cpu_instrs.gb")}
      >
        CPU All Tests
      </Dropdown.Item>
      <Dropdown.Item
        onClick={async () => await readFile(props, gameboy, "Dr. Mario.gb")}
      >
        Dr. Mario
      </Dropdown.Item>
      <Dropdown.Item
        onClick={async () =>
          await readFile(props, gameboy, "Super Mario Land.gb")
        }
      >
        Super Mario Land
      </Dropdown.Item>
      <Dropdown.Item
        onClick={async () => await readFile(props, gameboy, "Tetris.gb")}
      >
        Tetris
      </Dropdown.Item>
      <Dropdown.Item
        onClick={async () =>
          await readFile(props, gameboy, "Kirby's Dream Land.gb")
        }
      >
        Kirby's Dream Land
      </Dropdown.Item>
      <Dropdown.Item
        onClick={async () => await readFile(props, gameboy, "Zelda.gb")}
      >
        The Legend of Zelda Link's Awakening
      </Dropdown.Item>
      <Dropdown.Item
        onClick={async () => await readFile(props, gameboy, "Pokemon Blue.gb")}
      >
        Pokemon Blue
      </Dropdown.Item>
      <Dropdown.Item onClick={() => openFileSelector()}>
        Pick File...
      </Dropdown.Item>
    </StyledDropdownButton>
  );
};

const readFile = async (
  props: Props,
  gameboy: RustGameboy | null,
  fileName: string
) => {
  if (!gameboy?.Emulator) return;

  const response = await fetch(`/roms/${fileName}`);
  const blob = await response.blob();
  const buffer = await blob.arrayBuffer();
  const bytes = new Uint8Array(buffer);
  const key = fileName.replace(/.gb$/, "");
  const fileData = getFileData(key);
  if (fileData === null) {
    const emulator = new gameboy.Emulator(bytes);
    props.setCurrentGame(key);
    props.loadRom(emulator);
  } else {
    const emulator = gameboy.Emulator.from_save_data(bytes, fileData);
    props.setCurrentGame(key);
    props.loadRom(emulator);
  }
};

const pickFile = async (
  props: Props,
  gameboy: RustGameboy | null,
  file: File
) => {
  if (!gameboy?.Emulator) {
    return;
  }

  const buffer = await file.arrayBuffer();
  const bytes = new Uint8Array(buffer);
  let emulator = new gameboy.Emulator(bytes);
  props.loadRom(emulator);
};

const getFileData = (key: string): Uint8Array | null => {
  if (!window || !window.localStorage) {
    return null;
  }

  const item = window.localStorage.getItem(key);
  if (item === null) {
    return null;
  }

  return new Uint8Array(JSON.parse(item));
};

const mapStateToProps = (state: State) => {
  return {
    emulator: state.gameboy.emulator,
  };
};

const mapDispatchToProps = (dispatch: any) => ({
  loadRom: (emulator: Emulator) => dispatch(loadRom(emulator)),
  setCurrentGame: (currentGame: string) =>
    dispatch(setCurrentGame(currentGame)),
});

export default connect(mapStateToProps, mapDispatchToProps)(RomLoader);
