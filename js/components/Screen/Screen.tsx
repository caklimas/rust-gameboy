import React, { useCallback, useEffect, useState } from 'react';
import { connect } from 'react-redux';
import styled from 'styled-components';
import chunk from 'chunk';
import { loadWasm } from '../../helpers/wasm';
import {
  SET_RUST_GAMEBOY,
  setRustGameboy
} from '../../redux/actions/rustGameboy';
import { State } from '../../redux/state/state';
import { RustGameboy } from '../../redux/state/rustGameboy';
import { mediaMinMd } from '../../constants/screenSizes';
import { Emulator, EmulatorState } from 'gameboy';
import { useSelector } from 'react-redux';
import { useDispatch } from 'react-redux';

type Props = {
  className?: string;
  width: number;
  height: number;
  pixelSize: number;
};

interface ScreenState {
  width: number;
  height: number;
  bytesPerRow: number;
  bytesPerColumn: number;
  timestamp: number;
  emptyAudioBuffers: AudioBuffer[];
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

export function Screen(props: Props) {
  let bytesPerColumn = props.pixelSize * 4;
  let bytesPerRow = bytesPerColumn * props.width;

  const [requestId, setRequestId] = useState<number>(0);
  const [canvas, setCanvas] = useState<HTMLCanvasElement | null>(null);
  const [state, setState] = useState<ScreenState>({
    width: props.width * props.pixelSize,
    height: props.height * props.pixelSize,
    bytesPerRow,
    bytesPerColumn,
    timestamp: 0,
    emptyAudioBuffers: []
  });

  const [wasm, setWasm] = useState<RustGameboy | null>(null);
  const dispatch = useDispatch();

  const [emulatorState, setEmulatorState] = useState<
    typeof EmulatorState | null
  >(null);
  const emulator = useSelector<State, Emulator>(
    (state) => state.gameboy.emulator!
  );

  const renderScreen = useCallback(() => {
    const screen = emulator.get_screen();
    const chunked = chunk(screen, 3);
    const ctx = canvas!.getContext('2d')!;
    const imageData = ctx.createImageData(state.width, state.height);
    const data = imageData.data;
    for (let i = 0; i < chunked.length; i++) {
      let rgb = chunked[i];
      let x = i % props.width;
      let y = Math.floor(i / props.width);
      let yOffset = y * state.bytesPerRow * props.pixelSize;
      for (let rowNum = 0; rowNum < props.pixelSize; rowNum++) {
        let rowOffset = yOffset + rowNum * state.bytesPerRow;
        let xOffset = x * state.bytesPerColumn;

        for (let colNum = 0; colNum < props.pixelSize; colNum++) {
          let colOffset = xOffset + colNum * 4;
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
  }, [emulator, canvas, state]);

  const animate = useCallback(() => {
    setRequestId(requestAnimationFrame(animate));

    if (!canvas || !emulator || !emulatorState) return;

    while (true) {
      const event = emulator.clock_until_event(maxCycles);
      if (event && event === emulatorState.AudioFull) {
        //playAudio();
      } else if (event === emulatorState.MaxCycles) {
        break;
      }
    }

    renderScreen();
  }, [canvas, emulator, emulatorState, setRequestId, renderScreen]);

  const playAudio = useCallback(() => {
    const audio = emulator.get_audio_buffer();
    let audioBuffer: AudioBuffer;
    if (state.emptyAudioBuffers.length === 0) {
      audioBuffer = audioCtx.createBuffer(2, sampleCount, sampleRate * 2);
    } else {
      audioBuffer = state.emptyAudioBuffers[state.emptyAudioBuffers.length - 1];
      setState({
        ...state,
        emptyAudioBuffers: state.emptyAudioBuffers.slice(0, -1)
      });
    }

    audioBuffer.getChannelData(0).set(audio);
    audioBuffer.getChannelData(1).set(audio);

    const node = audioCtx.createBufferSource();
    node.connect(audioCtx.destination);
    node.buffer = audioBuffer;
    node.onended = () => {
      setState({
        ...state,
        emptyAudioBuffers: [...state.emptyAudioBuffers, audioBuffer]
      });
    };

    const playTimestamp = Math.max(
      audioCtx.currentTime + latency,
      state.timestamp
    );
    node.start(playTimestamp);

    setState({
      ...state,
      timestamp: playTimestamp + sampleCount / 2 / sampleRate
    });
  }, [state, setState]);

  useEffect(() => {
    const fetchWasm = async () => {
      const wasm = await loadWasm();
      setWasm(wasm);
      setEmulatorState(wasm.EmulatorState);
      dispatch(setRustGameboy(wasm));
      animate();
      console.log('Loaded WASM');
    };

    fetchWasm();
    return () => {
      if (requestId) {
        cancelAnimationFrame(requestId);
      }
    };
  }, [wasm]);

  return (
    <GameboyScreenFlex>
      <StyledCanvas
        ref={setCanvas}
        width={props.width * props.pixelSize}
        height={props.height * props.pixelSize}
      />
    </GameboyScreenFlex>
  );
}
