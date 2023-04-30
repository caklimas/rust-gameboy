import styled from "styled-components";
import Button from "react-bootstrap/Button";

type ButtonType = "circle" | "directional" | "start-select";

const StyledButton = styled(Button)`
  font-size: 15px;
  touch-action: none;
`;

const CircleButton = styled(StyledButton)`
  border-radius: 50%;
  width: 38px;
`;

const DirectionalButton = styled(StyledButton)`
  width: 60px;
`;

const StartSelectButton = styled(StyledButton)`
  font-size: 1rem;
  width: 75px;
`;

interface Props {
  pressed: boolean;
  text: string;
  type: ButtonType;

  onTouchStart?: (e: React.TouchEvent<HTMLElement>) => void;
  onTouchEnd?: (e: React.TouchEvent<HTMLElement>) => void;
  onTouchCancel?: (e: React.TouchEvent<HTMLElement>) => void;
}

const ControlButton = ({
  type,
  pressed,
  text,
  onTouchStart,
  onTouchEnd,
  onTouchCancel,
}: Props) => {
  const getVariant = (pressed: boolean): string =>
    pressed ? "primary" : "secondary";

  const getButtonComponent = () => {
    switch (type) {
      case "circle":
        return CircleButton;
      case "directional":
        return DirectionalButton;
      case "start-select":
        return StartSelectButton;
      default:
        throw new Error(`Invalid button type ${type}`);
    }
  };

  const ButtonComponent = getButtonComponent();
  return (
    <ButtonComponent
      disabled
      variant={getVariant(pressed)}
      onTouchStart={onTouchStart}
      onTouchEnd={onTouchEnd}
      onTouchCancel={onTouchCancel}
    >
      {text}
    </ButtonComponent>
  );
};

export default ControlButton;
