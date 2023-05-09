import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Provider } from "react-redux";
import { getStore } from "./redux/store";
import App from "./components/App/App";
import "./index.scss";
import "react-tabs/style/react-tabs.css";

const root = createRoot(document.getElementById("root")!);
root.render(
  <StrictMode>
    <Provider store={getStore()}>
      <App />
    </Provider>
  </StrictMode>
);
