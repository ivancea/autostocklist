import { Button } from "@mui/material";
import { SnackbarKey, SnackbarProvider } from "notistack";
import React from "react";
import ReactDOM from "react-dom";
import App from "./components/App/App";
import "./index.css";
import { isNil } from "./utils";

const notistackRef = React.createRef<SnackbarProvider>();
const onClickDismiss = (key: SnackbarKey) => () => {
  if (!isNil(notistackRef.current)) {
    notistackRef.current.closeSnackbar(key);
  }
};

ReactDOM.render(
  <React.StrictMode>
    <SnackbarProvider
      ref={notistackRef}
      action={(key) => <Button onClick={onClickDismiss(key)}>Dismiss</Button>}
    >
      <App />
    </SnackbarProvider>
  </React.StrictMode>,
  document.getElementById("root")
);
