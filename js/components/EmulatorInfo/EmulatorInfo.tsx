import { useSelector } from "react-redux";
import { Emulator } from "gameboy";
import { State } from "../../redux/state/state";
import { useCallback, useState } from "react";
import { Button, Modal } from "react-bootstrap";
import chunk from "chunk";
import styled from "styled-components";
import { TileInfo } from "./TileInfo";

type Props = {
    show: boolean,
    setShow: (show: boolean) => void;
}

export function EmulatorInfo({ show, setShow }: Props) {
    const emulator = useSelector<State, Emulator>(state => state.gameboy.emulator!);
    const handleClose = useCallback(() => setShow(false), [setShow]);
    if (!emulator) return null;

    const cartridgeType = JSON.parse(emulator.get_header_info()).header.cartridge_type;
    const colors = chunk(emulator.get_tiles(), 3);
    const tiles = chunk(colors, 64);
    console.log({ tileLEngth: tiles.length });

    const CanvasContainer = styled.div`
        display: grid;
        grid-template-rows: repeat(${Math.ceil(tiles.length / 16)}, 10px);
        grid-template-columns: repeat(16, 10px);
    `;

    return (
        <>
            <Modal show={show} onHide={handleClose}>
                <Modal.Header closeButton>
                    <Modal.Title>
                        Emulator Information
                    </Modal.Title>
                </Modal.Header>
                <Modal.Body>
                    <p>
                        Cartridge Type: {cartridgeType}
                    </p>
                    <CanvasContainer>
                        {
                            tiles.map(tile => (
                                <TileInfo tile={tile} />
                            ))
                        }
                    </CanvasContainer>
                </Modal.Body>
                <Modal.Footer>
                    <Button variant='secondary' onClick={handleClose}>
                        Close
                    </Button>
                </Modal.Footer>
            </Modal>
        </>
    );
}