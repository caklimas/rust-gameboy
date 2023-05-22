import { useSelector } from 'react-redux';
import { Emulator } from 'gameboy';
import { State } from '../../redux/state/state';
import { useCallback, useMemo, useState } from 'react';
import { Button, Modal, Tab, Tabs } from 'react-bootstrap';
import chunk from 'chunk';
import styled from 'styled-components';
import { TileInfo } from './TileInfo';
import { CANVAS_WIDTH } from './constants';

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

  const header = useMemo(
    () => JSON.parse(emulator.get_header_info()).header,
    [emulator]
  );

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
          <Tabs defaultActiveKey="cartridge-info" className="mb-3">
            <Tab eventKey="cartridge-info" title="Cartridge Info">
              <p>Cartridge Type: {header.cartridge_type}</p>
              <p>CGB Mode: {header.cgb_mode}</p>
            </Tab>
            <Tab eventKey="vram-viewer" title="VRAM Viewer">
              <CanvasContainer>
                {tiles.map((tile) => (
                  <TileInfo tile={tile} />
                ))}
              </CanvasContainer>
            </Tab>
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
