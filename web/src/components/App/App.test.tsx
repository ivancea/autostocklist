import { render } from "@testing-library/react";
import { SnackbarProvider } from "notistack";
import React from "react";
import App from "./App";

test("renders learn react link", () => {
  render(
    <React.StrictMode>
      <SnackbarProvider>
        <App />
      </SnackbarProvider>
    </React.StrictMode>
  );

  /*const linkElement = screen.getByText(/learn react/i);
  expect(linkElement).toBeInTheDocument();*/
});
