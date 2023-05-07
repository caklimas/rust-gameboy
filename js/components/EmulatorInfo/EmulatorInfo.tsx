import { useSelector } from "react-redux";
import { Emulator } from "gameboy";
import { State } from "../../redux/state/state";
import { useCallback, useState } from "react";
import { Button, Modal } from "react-bootstrap";
import chunk from "chunk";
import styled from "styled-components";

type Props = {
    show: boolean,
    setShow: (show: boolean) => void;
}

const StyledCanvas = styled.canvas`
  border: 1px solid #000000;
  margin: 20px 20px 0px;
  width: 320px;
`;

export function EmulatorInfo({ show, setShow }: Props) {
    const emulator = useSelector<State, Emulator>(state => state.gameboy.emulator!);
    const [canvas, setCavnas] = useState<HTMLCanvasElement | null>(null);
    const handleClose = useCallback(() => setShow(false), [setShow]);
    if (!emulator) return null;

    const cartridgeType = JSON.parse(emulator.get_header_info()).header.cartridge_type;

    const renderScreen = () => {
        if (!canvas) {
            return;
        }

        const ctx = canvas.getContext("2d")!;
        const imageData = ctx.createImageData(8, 8);
        const data = imageData.data;
        console.log({ dataLength: data.length });
        const colors = chunk(emulator.get_tiles(), 3);
        const tiles = chunk(colors, 64);

        for (let columnOffset = 0; columnOffset < 1; columnOffset++) {
            const tile = tiles[columnOffset + 1];

            // tiles are 8 x 8 pixels each having a color
            const tileRows = chunk(tile, 8);
            tileRows.forEach((tileRow, rowOffset) => {
                tileRow.forEach((color, colorOffset) => {
                    const offset = (colorOffset * 4) + (32 * rowOffset) + (192 * columnOffset);
                    color.forEach((rgb, rgbOffset) => {
                        data[offset + rgbOffset] = rgb;
                        console.log({ offset: offset + rgbOffset, rgb });
                    });

                    data[offset + color.length] = 255;
                    console.log({ offset: offset + color.length, alpha: 255 });
                });
            });
        }

        ctx.putImageData(imageData, 0, 0);
    };

    renderScreen();

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
                    <StyledCanvas
                        ref={setCavnas}
                        width={100}
                        height={100}
                    />
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