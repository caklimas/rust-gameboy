import { ReactNode } from 'react';

interface Props {
    children?: ReactNode;
    column: number;
    row: number;
};

const GridCell = ({
    children,
    column,
    row
}: Props) => {
    return (
        <div style={{ gridColumn: column, gridRow: row }}>
            {children}
        </div>
    );
};

export default GridCell;
