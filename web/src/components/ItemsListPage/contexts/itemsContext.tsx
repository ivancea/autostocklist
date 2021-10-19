import React, { useContext, useState } from "react";
import { Item } from "../../../types/item";

export type ItemsContextType = {
  items: Item[];
  setItems: React.Dispatch<React.SetStateAction<Item[]>>;
};

const ItemsContext = React.createContext<ItemsContextType>({
  items: [],
  setItems: () => {
    // Do nothing
  },
});

export const ItemsContextProvider = ({
  children,
}: {
  children: React.ReactNode;
}): React.ReactElement => {
  const [items, setItems] = useState<Item[]>([]);

  return (
    <ItemsContext.Provider value={{ items, setItems }}>
      {children}
    </ItemsContext.Provider>
  );
};

export function useItems(): ItemsContextType {
  return useContext(ItemsContext);
}
