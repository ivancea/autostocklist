import AddIcon from "@mui/icons-material/Add";
import { CircularProgress, Fab } from "@mui/material";
import {
  DataGrid,
  GridColumns,
  GridRowParams,
  GridSortModel,
} from "@mui/x-data-grid";
import { useSnackbar } from "notistack";
import React, { useCallback, useEffect, useMemo, useState } from "react";
import { createItem, getItems, updateItem } from "../../../api";
import { stringifyError } from "../../../errors";
import { Item } from "../../../types/item";
import RemoveItem from "../actions/RemoveItem";
import { useItems } from "../contexts/itemsContext";
import AddStock from "./../actions/AddStock";
import RemoveStock from "./../actions/RemoveStock";

export default function ItemsList(): React.ReactElement {
  const { enqueueSnackbar } = useSnackbar();
  const { items, setItems } = useItems();

  // Loaders
  const [loadingItems, setLoadingItems] = useState(false);
  const [creatingItem, setCreatingItem] = useState(false);

  const [sortModel, setSortModel] = useState<GridSortModel>([
    {
      field: "name",
      sort: "asc",
    },
  ]);

  useEffect(() => {
    setLoadingItems(true);
    getItems()
      .then(setItems)
      .catch((e) =>
        enqueueSnackbar(`Error updating stock: ${stringifyError(e)}`, {
          persist: true,
          variant: "error",
        })
      )
      .finally(() => setLoadingItems(false));
  }, [enqueueSnackbar, setItems]);

  const updateItemStock = useCallback(
    (item: Item, newStock: number) => {
      item.stock = newStock;
      setItems((items) => [...items]);
    },
    [setItems]
  );

  const addItem = useCallback(() => {
    setCreatingItem(true);
    createItem()
      .then((item) => {
        setItems((items) => [...items, item]);
      })
      .catch((e) =>
        enqueueSnackbar(`Error creating item: ${stringifyError(e)}`, {
          persist: true,
          variant: "error",
        })
      )
      .finally(() => setCreatingItem(false));
  }, [enqueueSnackbar, setItems]);

  const editItem = useCallback(
    (params: GridRowParams) => {
      const editedItem = params.row as Item;

      updateItem(editedItem)
        .then((item) => {
          setItems((items) => items.map((i) => (i.id === item.id ? item : i)));
        })
        .catch((e) =>
          enqueueSnackbar(`Error updating item: ${stringifyError(e)}`, {
            persist: true,
            variant: "error",
          })
        );
    },
    [enqueueSnackbar, setItems]
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
      {
        field: "name",
        headerName: "Nombre",
        type: "string",
        width: 150,
        editable: true,
      },
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
      {
        field: "minStock",
        headerName: "Min",
        type: "number",
        width: 75,
        editable: true,
      },
      {
        field: "maxStock",
        headerName: "Max",
        type: "number",
        width: 75,
        editable: true,
      },
      {
        field: "maxStock",
        type: "actions",
        width: 75,
        getActions: (params: GridRowParams) => {
          const item = params.row as Item;
          return [<RemoveItem key="remove" item={item} />];
        },
      },
    ],
    [updateItemStock]
  );

  return (
    <div>
      <Fab
        color="primary"
        aria-label="add"
        title="Add item"
        onClick={addItem}
        disabled={creatingItem}
      >
        {creatingItem ? <CircularProgress /> : <AddIcon />}
      </Fab>
      <div style={{ height: "60vh", maxWidth: "800px", margin: "0 auto" }}>
        <DataGrid
          loading={loadingItems}
          rows={items}
          columns={columns}
          disableSelectionOnClick
          sortModel={sortModel}
          editMode="row"
          onSortModelChange={setSortModel}
          onRowEditStop={editItem}
          onRowEditCommit={() => {
            console.log("a");
          }}
        />
      </div>
    </div>
  );
}
