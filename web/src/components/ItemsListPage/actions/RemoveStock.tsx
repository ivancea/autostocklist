import RemoveCircleIcon from "@mui/icons-material/RemoveCircle";
import {
  CircularProgress,
  Dialog,
  DialogTitle,
  List,
  ListItem,
  ListItemText,
} from "@mui/material";
import { GridActionsCellItem } from "@mui/x-data-grid";
import { useSnackbar } from "notistack";
import React, { useState } from "react";
import { updateStockLoss } from "../../../api";
import { stringifyError } from "../../../errors";
import { Item } from "../../../types/item";

type Props = {
  item: Item;
  onNewStock: (newStock: number) => void;
};

export default function RemoveStock({
  item,
  onNewStock,
}: Props): React.ReactElement {
  const { enqueueSnackbar } = useSnackbar();
  const [openDialog, setOpenDialog] = useState(false);
  const [working, setWorking] = useState(false);

  const handleSelectValue = (value: number) => {
    setWorking(true);

    updateStockLoss(item.id, value)
      .then(onNewStock)
      .catch((e) => {
        enqueueSnackbar(`Error updating stock: ${stringifyError(e)}`, {
          persist: true,
          variant: "error",
        });
      })
      .finally(() => setOpenDialog(false))
      .finally(() => setWorking(false));
  };

  return (
    <>
      <GridActionsCellItem
        icon={<RemoveCircleIcon color="warning" />}
        label="Remove stock"
        title="Remove stock"
        onClick={() => setOpenDialog(true)}
      />

      <Dialog open={openDialog}>
        <DialogTitle>
          <div>Select stock quantity to reduce.</div>
          <div>
            Negative values exist to fix errors of the current day, not to
            increment stock.
          </div>
        </DialogTitle>

        {working ? (
          <CircularProgress />
        ) : (
          <List>
            {[1, 2, 5, 10, -1, -5].map((value) => (
              <ListItem
                key={value}
                button
                disabled={item.stock - value < 0}
                onClick={() => handleSelectValue(value)}
              >
                <ListItemText primary={value} />
              </ListItem>
            ))}
            <ListItem button onClick={() => setOpenDialog(false)}>
              <ListItemText primary="Cancel" />
            </ListItem>
          </List>
        )}
      </Dialog>
    </>
  );
}
