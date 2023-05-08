import chunk from "chunk";
import { useState } from "react";
import styled from "styled-components";

type Props = {
    tile: number[][]
};

const TILE_LENGTH = 8;

const StyledCanvas = styled.canvas`
  border: 1px solid #000000;
  margin: 20px 20px 0px;
`;

export function TileInfo({ tile }: Props) {
    const [canvas, setCavnas] = useState<HTMLCanvasElement | null>(null);

    const renderScreen = () => {
        if (!canvas) {
            return;
        }

        const ctx = canvas.getContext("2d")!;
        const imageData = ctx.createImageData(TILE_LENGTH, TILE_LENGTH);
        const data = imageData.data;

        // tiles are 8 x 8 pixels each having a color
        const tileRows = chunk(tile, 8);
        tileRows.forEach((tileRow, rowOffset) => {
            tileRow.forEach((color, colorOffset) => {
                const offset = (colorOffset * 4) + (32 * rowOffset);
                color.forEach((rgb, rgbOffset) => {
                    data[offset + rgbOffset] = rgb;
                });

                data[offset + color.length] = 255;
            });
        });

        ctx.putImageData(imageData, 0, 0);
    };

    renderScreen();

    return (
        <StyledCanvas ref={setCavnas} width={TILE_LENGTH} height={TILE_LENGTH} />
    );
}