import chunk from "chunk";
import { useState } from "react";
import styled from "styled-components";
import { CANVAS_WIDTH, TILE_LENGTH } from "./constants";

type Props = {
  tile: number[][];
};

const StyledCanvas = styled.canvas`
  border: 1px solid #000000;
`;

export function TileInfo({ tile }: Props) {
  const [canvas, setCavnas] = useState<HTMLCanvasElement | null>(null);

  const renderScreen = () => {
    if (!canvas) {
      return;
    }

    const newCanvas = document.createElement("canvas");
    newCanvas.width = 8;
    newCanvas.height = 8;

    const ctx = newCanvas.getContext("2d")!;
    const imageData = ctx.createImageData(TILE_LENGTH, TILE_LENGTH);
    const data = imageData.data;

    // tiles are 8 x 8 pixels each having a color
    const tileRows = chunk(tile, 8);
    tileRows.forEach((tileRow, rowOffset) => {
      tileRow.forEach((color, colorOffset) => {
        const offset = colorOffset * 4 + 32 * rowOffset;
        color.forEach((rgb, rgbOffset) => {
          data[offset + rgbOffset] = rgb;
        });

        data[offset + color.length] = 255;
      });
    });

    ctx.putImageData(imageData, 0, 0);

    const ctx2 = canvas.getContext("2d")!;
    ctx2.drawImage(newCanvas, 0, 0, CANVAS_WIDTH, CANVAS_WIDTH);
  };

  renderScreen();

  return (
    <StyledCanvas ref={setCavnas} width={CANVAS_WIDTH} height={CANVAS_WIDTH} />
  );
}
