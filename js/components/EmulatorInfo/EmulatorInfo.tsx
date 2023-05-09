import { useSelector } from "react-redux";
import { Emulator } from "gameboy";
import { State } from "../../redux/state/state";
import { useCallback, useState } from "react";
import { Button, Modal } from "react-bootstrap";
import { Tab, TabList, TabPanel, Tabs } from "react-tabs";
import chunk from "chunk";
import styled from "styled-components";
import { TileInfo } from "./TileInfo";
import { CANVAS_WIDTH } from "./constants";

type Props = {
  show: boolean;
  setShow: (show: boolean) => void;
};

const GRID_GAP = CANVAS_WIDTH + 3;

export function EmulatorInfo({ show, setShow }: Props) {
  const emulator = useSelector<State, Emulator>(
    (state) => state.gameboy.emulator!
  );
  const handleClose = useCallback(() => setShow(false), [setShow]);
  if (!emulator) return null;

  const cartridgeType = JSON.parse(emulator.get_header_info()).header
    .cartridge_type;
  const colors = chunk(emulator.get_tiles(), 3);
  const tiles = chunk(colors, 64);
  console.log({ tileLEngth: tiles.length });

  const CanvasContainer = styled.div`
    display: grid;
    grid-template-rows: repeat(${Math.ceil(tiles.length / 16)}, ${GRID_GAP}px);
    grid-template-columns: repeat(16, ${GRID_GAP}px);
  `;

  return (
    <>
      <Modal show={show} onHide={handleClose} backdrop="static">
        <Modal.Header closeButton>
          <Modal.Title>Emulator Information</Modal.Title>
        </Modal.Header>
        <Modal.Body>
          <Tabs>
            <TabList>
              <Tab>Cartridge Info</Tab>
              <Tab>VRAM Viewer</Tab>
            </TabList>

            <TabPanel>
              <p>Cartridge Type: {cartridgeType}</p>
            </TabPanel>
            <TabPanel>
              <CanvasContainer>
                {tiles.map((tile) => (
                  <TileInfo tile={tile} />
                ))}
              </CanvasContainer>
            </TabPanel>
          </Tabs>
        </Modal.Body>
        <Modal.Footer>
          <Button variant="secondary" onClick={handleClose}>
            Close
          </Button>
        </Modal.Footer>
      </Modal>
    </>
  );
}
