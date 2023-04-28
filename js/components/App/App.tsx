import styled from "styled-components";
import Gameboy from "../Gameboy/Gameboy";
import RomLoader from "../RomLoader/RomLoader";
import { CartridgeInfo } from "../CartridgeInfo/CartridgeInfo";

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
      <CartridgeInfo />
    </StyledApp>
  );
};

export default App;
