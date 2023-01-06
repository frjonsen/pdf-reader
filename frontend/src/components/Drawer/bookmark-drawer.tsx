import {
  Alert,
  AlertTitle,
  Box,
  Button,
  CircularProgress,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  Divider,
  IconButton,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  TextField,
  Typography,
} from "@mui/material";
import SouthIcon from "@mui/icons-material/South";
import NorthIcon from "@mui/icons-material/North";
import useAxios from "axios-hooks";
import axios from "axios";
import { useState } from "react";
import BookmarkRemove from "@mui/icons-material/BookmarkRemove";
import BookmarkAdd from "@mui/icons-material/BookmarkAdd";
import { Bookmark } from "../../models";

interface BookmarkDrawerProps {
  setPage: (page: number) => void;
  documentId: string;
  currentPage: number;
}

interface AddBookmarkDialogProps {
  refreshBookmarks: () => void;
  open: boolean;
  page: number;
  document: string;
  close: () => void;
}

enum BookmarkSortKey {
  Page,
  AddedDate,
}

enum BookmarkSortOrder {
  Ascending,
  Descending,
}

function AddBookmarkDialog(props: AddBookmarkDialogProps): JSX.Element {
  const [description, setDescription] = useState("");

  const [
    { data: addResponse, loading: addLoading, error: addError },
    addBookmark,
  ] = useAxios(
    {
      url: `/api/documents/${props.document}/bookmarks`,
      method: "POST",
    },
    { manual: true }
  );

  async function addNewBookmark() {
    try {
      await addBookmark({
        data: { page: props.page, description },
      });
      props.refreshBookmarks();
      props.close();
    } catch (e) {}
  }

  return (
    <Dialog open={props.open}>
      <DialogTitle>Bookmark page {props.page}</DialogTitle>
      <DialogContent>
        <>
          {addError && (
            <Alert severity="error">
              <AlertTitle>{addError?.message}</AlertTitle>
              {addError?.response?.data}
            </Alert>
          )}
          <TextField
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            autoFocus
            margin="dense"
            name="description"
            label="Description"
            fullWidth
            variant="standard"
          />
        </>
      </DialogContent>
      <DialogActions>
        {addLoading && <CircularProgress />}
        <Button onClick={props.close}>Cancel</Button>
        <Button disabled={description === ""} onClick={addNewBookmark}>
          Add
        </Button>
      </DialogActions>
    </Dialog>
  );
}

export default function BookmarkDrawer({
  setPage,
  documentId,
  currentPage,
}: BookmarkDrawerProps): JSX.Element {
  const [addDialogOpen, setAddDialogOpen] = useState(false);
  const [bookmarkSortKey, setBookmarkSortKey] = useState(BookmarkSortKey.Page);
  const [bookmarkSortOrder, setBookmarkSortOrder] = useState(
    BookmarkSortOrder.Ascending
  );

  const [
    { data: bookmarks, loading: bookmarksLoading, error: bookmarksError },
    refetchBookmarks,
  ] = useAxios<Array<Bookmark>>(`/api/documents/${documentId}/bookmarks`);

  async function deleteBookmark(bookmarkId: string) {
    await axios.delete(`/api/documents/${documentId}/bookmarks/${bookmarkId}`);
    refetchBookmarks();
  }

  const sortedBookmarks = bookmarks?.sort((a, b) => {
    const [aVal, bVal] =
      bookmarkSortKey === BookmarkSortKey.Page
        ? [a.page, b.page]
        : [a.added_on, b.added_on];

    if (aVal === bVal) return 0;
    if (bookmarkSortOrder === BookmarkSortOrder.Ascending) {
      return aVal < bVal ? -1 : 1;
    } else {
      return aVal > bVal ? -1 : 1;
    }
  });

  function setSorting(key: BookmarkSortKey) {
    if (bookmarkSortKey === key) {
      setBookmarkSortOrder(
        bookmarkSortOrder === BookmarkSortOrder.Ascending
          ? BookmarkSortOrder.Descending
          : BookmarkSortOrder.Ascending
      );
    }

    setBookmarkSortKey(key);
  }

  function SortIcon({ forKey }: { forKey: BookmarkSortKey }): JSX.Element {
    if (forKey !== bookmarkSortKey) {
      return <></>;
    }
    return bookmarkSortOrder === BookmarkSortOrder.Ascending ? (
      <NorthIcon />
    ) : (
      <SouthIcon />
    );
  }

  return (
    <>
      <AddBookmarkDialog
        open={addDialogOpen}
        refreshBookmarks={refetchBookmarks}
        document={documentId}
        page={currentPage}
        close={() => setAddDialogOpen(false)}
      />
      <List>
        <ListItem disablePadding>
          <ListItemButton onClick={() => setAddDialogOpen(true)}>
            <ListItemIcon>
              <BookmarkAdd />
            </ListItemIcon>
            <ListItemText primary="Add new bookmark" />
          </ListItemButton>
        </ListItem>
      </List>
      <Divider />
      <List>
        <ListItem>
          <Box
            sx={{ width: "100%" }}
            justifyContent="space-evenly"
            display="flex"
          >
            <Button onClick={() => setSorting(BookmarkSortKey.AddedDate)}>
              Date
              <SortIcon forKey={BookmarkSortKey.AddedDate} />
            </Button>
            <Button onClick={() => setSorting(BookmarkSortKey.Page)}>
              Page
              <SortIcon forKey={BookmarkSortKey.Page} />
            </Button>
          </Box>
        </ListItem>
        {bookmarksError && (
          <ListItem>
            <ListItemText>
              <Typography>
                {bookmarksError.message} - {bookmarksError.response?.data}
              </Typography>
            </ListItemText>
          </ListItem>
        )}
        {bookmarksLoading && (
          <ListItem alignItems="center">
            <CircularProgress />
          </ListItem>
        )}
        {bookmarks &&
          bookmarks.map((b) => (
            <ListItem dense key={b.id}>
              <ListItemButton onClick={() => setPage(b.page)}>
                <ListItemText>
                  {b.page} {b.description}
                </ListItemText>
                <ListItemIcon>
                  <IconButton onClick={() => deleteBookmark(b.id)}>
                    <BookmarkRemove />
                  </IconButton>
                </ListItemIcon>
              </ListItemButton>
            </ListItem>
          ))}
      </List>
    </>
  );
}
