import React from "react";
import { useMediaQuery } from 'react-responsive';
import { mobileMediaQuery } from '../../helpers/mediaQueries';
import MobileControls from './MobileControls/MobileControls';
import DesktopControls from './DesktopControls/DesktopControls';

const Controls = () => {
    const isMobile = useMediaQuery(mobileMediaQuery);
    return isMobile ? (
        <MobileControls />
    ) : (
        <DesktopControls />
    );
};

export default Controls;