import {
  DataGrid,
  GridColumns,
  GridRowParams,
  GridSortModel,
} from "@mui/x-data-grid";
import { useSnackbar } from "notistack";
import React, { useCallback, useEffect, useMemo, useState } from "react";
import { getItems } from "../../../api";
import { stringifyError } from "../../../errors";
import { Item } from "../../../types/item";
import { useItems } from "../contexts/itemsContext";
import AddStock from "./../actions/AddStock";
import RemoveStock from "./../actions/RemoveStock";

export default function ItemsList(): React.ReactElement {
  const { enqueueSnackbar } = useSnackbar();
  const { items, setItems } = useItems();
  const [loading, setLoading] = useState(false);
  const [sortModel, setSortModel] = useState<GridSortModel>([
    {
      field: "name",
      sort: "asc",
    },
  ]);

  useEffect(() => {
    setLoading(true);
    getItems()
      .then(setItems)
      .catch((e) =>
        enqueueSnackbar(`Error updating stock: ${stringifyError(e)}`, {
          persist: true,
          variant: "error",
        })
      )
      .finally(() => setLoading(false));
  }, [enqueueSnackbar, setItems]);

  const updateItemStock = useCallback(
    (item: Item, newStock: number) => {
      item.stock = newStock;
      setItems((items) => [...items]);
    },
    [setItems]
  );

  const columns: GridColumns = useMemo(
    () => [
      {
        field: "status",
        headerName: "",
        width: 50,
        renderCell: (params) => {
          const item = params.row as Item;

          return item.stock < item.minStock
            ? "🔴"
            : item.stock < item.minStock + (item.maxStock - item.minStock) / 3
            ? "⚠️"
            : "✔️";
        },
      },
      { field: "name", headerName: "Nombre", type: "string", width: 150 },
      { field: "stock", headerName: "Stock", type: "number", width: 75 },
      {
        field: "actions",
        type: "actions",
        width: 150,
        getActions: (params: GridRowParams) => {
          const item = params.row as Item;
          return [
            <AddStock
              key="add"
              item={item}
              onNewStock={(newStock) => updateItemStock(item, newStock)}
            />,
            <RemoveStock
              key="remove"
              item={item}
              onNewStock={(newStock) => updateItemStock(item, newStock)}
            />,
          ];
        },
      },
      { field: "minStock", headerName: "Min", type: "number", width: 75 },
      { field: "maxStock", headerName: "Max", type: "number", width: 75 },
    ],
    [updateItemStock]
  );

  return (
    <div style={{ height: "60vh", maxWidth: "800px", margin: "0 auto" }}>
      <DataGrid
        loading={loading}
        rows={items}
        columns={columns}
        disableSelectionOnClick
        sortModel={sortModel}
        onSortModelChange={(model) => setSortModel(model)}
      />
    </div>
  );
}
