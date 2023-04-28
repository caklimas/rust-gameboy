import { connect } from 'react-redux';
import styled from 'styled-components';
import { useMediaQuery } from 'react-responsive';
import gameboyDimensions from '../../constants/gameboy';
import { mobileMediaQuery } from '../../helpers/mediaQueries';
import Screen from '../Screen/Screen';
import Controls from '../Controls/Controls';
import { mediaMinMd } from '../../constants/screenSizes';
import { Emulator } from 'caklimas-rust-gameboy';
import { State } from '../../redux/state/state';
import { useBeforeunload } from 'react-beforeunload';

interface Props {
    emulator: Emulator,
    currentGame: string
};

const StyledGameboy = styled.div`
    background-color: #bababa;
    border-top-left-radius: 10px;
    border-top-right-radius: 10px;
    border-bottom-left-radius: 5px;
    border-bottom-right-radius: 25px;
    display: flex;
    flex-direction: column;
    height: 500px;
    margin-top: 20px;
    width: 350px;

    @media only screen and (min-width: ${mediaMinMd}px) {
        height: 700px;
        width: 540px;
    }
`;

const Gameboy = ({ emulator, currentGame }: Props) => {
    useBeforeunload(() => {
        if (emulator && currentGame) {
            saveToLocalStorage(currentGame, emulator.save());
        }
    });

    const isMobile = useMediaQuery(mobileMediaQuery);
    const pixelSize = isMobile ? 1 : 3;
    if (!emulator)
        return null;

    return (
        <StyledGameboy>
            <Screen
                width={gameboyDimensions.width}
                height={gameboyDimensions.height}
                pixelSize={pixelSize}
            />
            <Controls />
        </StyledGameboy>
    );
};

const saveToLocalStorage = (key: string, data: Uint8Array) => {
    if (!window || !key) {
        return;
    }

    window.localStorage.setItem(key, JSON.stringify(Array.from(data)));
};

const mapStateToProps = (state: State): Props => {
    return {
        emulator: state.gameboy.emulator!,
        currentGame: state.currentGame
    };
};

export default connect(mapStateToProps)(Gameboy);