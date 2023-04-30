import { useSelector } from "react-redux";
import { Emulator } from "gameboy";
import { State } from "../../redux/state/state";
import { useCallback, useState } from "react";
import { Button, Modal } from "react-bootstrap";

type Props = {
    show: boolean,
    setShow: (show: boolean) => void;
}

export function EmulatorInfo({ show, setShow }: Props) {
    const emulator = useSelector<State, Emulator>(state => state.gameboy.emulator!);
    const handleClose = useCallback(() => setShow(false), [setShow]);
    if (!emulator) return null;

    const cartridgeType = JSON.parse(emulator.get_header_info()).header.cartridge_type;

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