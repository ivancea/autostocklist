import React from "react";
import { ItemsContextProvider } from "./contexts/itemsContext";
import ItemsList from "./ItemsList/ItemsList";

export default function ItemsListPage(): React.ReactElement {
  return (
    <ItemsContextProvider>
      <div>
        <h1>Items List</h1>
        <ItemsList />
      </div>
    </ItemsContextProvider>
  );
}
