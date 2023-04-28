import { useSelector } from "react-redux";
import { Emulator } from "../../../pkg";
import { State } from "../../redux/state/state";

type Props = OwnProps | StateProps;

interface OwnProps { }

interface StateProps {
    emulator: Emulator;
}

export function CartridgeInfo(props: Props) {
    const emulator = useSelector<State, Emulator>(state => state.gameboy.emulator!);
    if (!emulator) return null;

    return (
        <div>
            {JSON.parse(emulator.get_header_info()).header.cartridge_type}
        </div>
    );
}