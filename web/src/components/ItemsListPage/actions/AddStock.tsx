import AddShoppingCartIcon from "@mui/icons-material/AddShoppingCart";
import {
  Dialog,
  DialogTitle,
  List,
  ListItem,
  ListItemText,
} from "@mui/material";
import { GridActionsCellItem } from "@mui/x-data-grid";
import { useSnackbar } from "notistack";
import React, { useState } from "react";
import { updateStockResupply } from "../../../api";
import { stringifyError } from "../../../errors";
import { Item } from "../../../types/item";

type Props = {
  item: Item;
  onNewStock: (newStock: number) => void;
};

export default function AddStock({
  item,
  onNewStock,
}: Props): React.ReactElement {
  const { enqueueSnackbar } = useSnackbar();
  const [openDialog, setOpenDialog] = useState(false);

  const handleSelectValue = (value: number) => {
    updateStockResupply(item.id, value)
      .then(onNewStock)
      .catch((e) => {
        enqueueSnackbar(`Error updating stock: ${stringifyError(e)}`, {
          persist: true,
          variant: "error",
        });
      })
      .finally(() => setOpenDialog(false));
  };

  return (
    <>
      <GridActionsCellItem
        icon={<AddShoppingCartIcon color="success" />}
        label="Add stock"
        onClick={() => setOpenDialog(true)}
      />

      <Dialog open={openDialog}>
        <DialogTitle>
          <div>Select stock quantity to increase.</div>
          <div>
            Negative values exist to fix errors of the current day, not to
            reduce stock.
          </div>
        </DialogTitle>
        <List>
          {[1, 2, 5, 10, -1, -5].map((value) => (
            <ListItem
              key={value}
              button
              disabled={item.stock + value < 0}
              onClick={() => handleSelectValue(value)}
            >
              <ListItemText primary={value} />
            </ListItem>
          ))}
          <ListItem button onClick={() => setOpenDialog(false)}>
            <ListItemText primary="Cancel" />
          </ListItem>
        </List>
      </Dialog>
    </>
  );
}
