import React from 'react';
import { connect } from 'react-redux';
import styled from 'styled-components';
import chunk from 'chunk';
import { loadWasm } from '../../helpers/wasm';
import { setRustGameboy } from '../../redux/actions/rustGameboy';
import { State } from '../../redux/state/state';
import { RustGameboy } from '../../redux/state/rustGameboy';
import { mediaMinMd } from '../../constants/screenSizes';
import { Emulator, EmulatorState } from 'caklimas-rust-gameboy';

type Props = OwnProps & StateProps & DispatchProps;

interface OwnProps {
    className?: string,
    width: number,
    height: number,
    pixelSize: number
}

interface StateProps {
    emulator: Emulator,
    EmulatorState: typeof EmulatorState
}

interface DispatchProps {
    setRustGameboy(rustGameboy: RustGameboy | undefined): void;
}

interface ScreenState {
    width: number,
    height: number,
    bytesPerRow: number,
    bytesPerColumn: number,
    timestamp: number,
    emptyAudioBuffers: AudioBuffer[]
}

const GameboyScreenFlex = styled.div`
    display: flex;
    justify-content: center; 
`;

const StyledCanvas = styled.canvas`
    border: 1px solid #000000;
    margin: 20px 20px 0px;
    width: 320px;

    @media only screen and (min-width: ${mediaMinMd}px) {
        width: 500px;
    }
`;

const maxCycles = 69_905;
const sampleRate = 44_100.0;
const sampleCount = 4096;
const latency = 0.032;
const audioCtx = new AudioContext();

class Screen extends React.Component<Props, ScreenState> {
    private canvas: HTMLCanvasElement | null;
    private request_id: number;

    async componentDidMount() {
        const wasm = await loadWasm();
        this.props.setRustGameboy(wasm);
        this.animate();
        console.log("Loaded WASM");
    }

    componentWillUnmount() {
        if (this.request_id) {
            cancelAnimationFrame(this.request_id);
        }
    }

    constructor(props: Props) {
        super(props);

        this.canvas = null;
        this.request_id = 0;

        let bytesPerColumn = props.pixelSize * 4;
        let bytesPerRow = bytesPerColumn * props.width;
        this.state = {
            width: props.width * props.pixelSize,
            height: props.height * props.pixelSize,
            bytesPerRow,
            bytesPerColumn,
            timestamp: 0,
            emptyAudioBuffers: []
        };
    }

    render() {
        return (
            <GameboyScreenFlex>
                <StyledCanvas
                    ref={this.setCanvasRef}
                    width={this.props.width * this.props.pixelSize}
                    height={this.props.height * this.props.pixelSize}
                />
            </GameboyScreenFlex>
        );
    }

    animate = () => {
        if (!this.canAnimate())
            return;

        while (true) {
            const event = this.props.emulator.clock_until_event(maxCycles);
            if (event === this.props.EmulatorState.AudioFull) {
                this.playAudio();
            } else if (event === this.props.EmulatorState.MaxCycles) {
                break;
            }
        }

        this.renderScreen();

        this.request_id = requestAnimationFrame(this.animate);
    }

    setCanvasRef = (element: HTMLCanvasElement) => {
        if (!element)
            return;
        this.canvas = element;
    }

    canAnimate = () => {
        return !!this.canvas && !!this.props.emulator;
    }

    playAudio = () => {
        const audio = this.props.emulator.get_audio_buffer();
        let audioBuffer: AudioBuffer;
        if (this.state.emptyAudioBuffers.length === 0) {
            audioBuffer = audioCtx.createBuffer(2, sampleCount, sampleRate * 2);
        } else {
            audioBuffer = this.state.emptyAudioBuffers[this.state.emptyAudioBuffers.length - 1];
            this.setState(prevState => (
                {
                    emptyAudioBuffers: prevState.emptyAudioBuffers.slice(0, -1)
                }
            ));
        }

        audioBuffer.getChannelData(0).set(audio);
        audioBuffer.getChannelData(1).set(audio);

        const node = audioCtx.createBufferSource();
        node.connect(audioCtx.destination);
        node.buffer = audioBuffer;
        node.onended = () => {
            this.setState(prevState => ({ emptyAudioBuffers: [...prevState.emptyAudioBuffers, audioBuffer] }));
        };

        const playTimestamp = Math.max(audioCtx.currentTime + latency, this.state.timestamp);
        node.start(playTimestamp);

        this.setState({ timestamp: playTimestamp + sampleCount / 2 / sampleRate });
    }

    renderScreen = () => {
        const screen = this.props.emulator.get_screen();
        const chunked = chunk(screen, 3);
        const ctx = this.canvas!.getContext('2d')!;
        const imageData = ctx.createImageData(this.state.width, this.state.height);
        const data = imageData.data;
        for (let i = 0; i < chunked.length; i++) {
            let rgb = chunked[i];
            let x = i % this.props.width;
            let y = Math.floor(i / this.props.width);
            let yOffset = y * this.state.bytesPerRow * this.props.pixelSize;
            for (let rowNum = 0; rowNum < this.props.pixelSize; rowNum++) {
                let rowOffset = yOffset + (rowNum * this.state.bytesPerRow);
                let xOffset = x * this.state.bytesPerColumn;

                for (let colNum = 0; colNum < this.props.pixelSize; colNum++) {
                    let colOffset = xOffset + (colNum * 4);
                    let offset = rowOffset + colOffset;
                    let color = 0;
                    while (color < rgb.length) {
                        data[offset + color] = rgb[color];
                        color++;
                    }

                    data[offset + color] = 255;
                }
            }
        }

        ctx.putImageData(imageData, 0, 0);
    }
}

const mapStateToProps = (state: State): StateProps => {
    return {
        emulator: state.gameboy.emulator!,
        EmulatorState: state.rustGameboy.EmulatorState!
    };
};

const mapDispatchToProps = (dispatch: any): DispatchProps => ({
    setRustGameboy: (rustGameboy: RustGameboy) => dispatch(setRustGameboy(rustGameboy))
});

export default connect(mapStateToProps, mapDispatchToProps)(Screen);