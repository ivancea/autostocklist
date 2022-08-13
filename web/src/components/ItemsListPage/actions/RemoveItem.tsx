import DeleteForeverIcon from "@mui/icons-material/DeleteForever";
import {
  Button,
  CircularProgress,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
} from "@mui/material";
import { GridActionsCellItem } from "@mui/x-data-grid";
import { useSnackbar } from "notistack";
import React, { useState } from "react";
import { removeItem } from "../../../api";
import { stringifyError } from "../../../errors";
import { Item } from "../../../types/item";
import { useItems } from "../contexts/itemsContext";

type Props = {
  item: Item;
};

export default function RemoveItem({ item }: Props): React.ReactElement {
  const { enqueueSnackbar } = useSnackbar();
  const [openDialog, setOpenDialog] = useState(false);
  const [working, setWorking] = useState(false);
  const { setItems } = useItems();

  const handleRemoveItem = () => {
    setWorking(true);

    removeItem(item)
      .then(() => {
        setItems((items) => items.filter((i) => i.id !== item.id));
      })
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
        icon={<DeleteForeverIcon color="error" />}
        label="Remove item"
        title="Remove item"
        onClick={() => setOpenDialog(true)}
      />

      <Dialog open={openDialog}>
        <DialogTitle>
          Are you sure you want to remove the item &quot;{item.name}&quot;?
        </DialogTitle>
        <DialogContent>
          <DialogContentText>
            This action is permanent, and all the history will be lost
          </DialogContentText>
        </DialogContent>

        {working ? (
          <CircularProgress />
        ) : (
          <DialogActions>
            <Button onClick={() => setOpenDialog(false)} autoFocus>
              Cancel
            </Button>
            <Button onClick={handleRemoveItem} autoFocus>
              Remove
            </Button>
          </DialogActions>
        )}
      </Dialog>
    </>
  );
}
