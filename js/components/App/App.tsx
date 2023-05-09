import styled from "styled-components";
import Gameboy from "../Gameboy/Gameboy";
import RomLoader from "../RomLoader/RomLoader";
import { EmulatorInfo } from "../EmulatorInfo/EmulatorInfo";
import { Button } from "react-bootstrap";
import { Tab, TabList, TabPanel, Tabs } from "react-tabs";

const StyledApp = styled.div`
  background-color: black;
  display: flex;
  justify-content: center;
  height: 100vh;
`;

const App = () => {
  return (
    <StyledApp>
      <RomLoader />
      <Gameboy />
    </StyledApp>
  );
};

export default App;
