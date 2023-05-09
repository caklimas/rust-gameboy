import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Provider } from "react-redux";
import { getStore } from "./redux/store";
import App from "./components/App/App";
import "./index.scss";

const root = createRoot(document.getElementById("root")!);
root.render(
  <StrictMode>
    <Provider store={getStore()}>
      <App />
    </Provider>
  </StrictMode>
);
