import MuiAppBar, { AppBarProps } from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Box from "@mui/material/Box";
import { Documents } from "./documents";
import { Uploader } from "./uploader";
import { Document } from "../models";
import PageControl, { PageControlProps } from "./pagecontrol";
import IconButton from "@mui/material/IconButton";
import MenuBook from "@mui/icons-material/MenuBook";
import Height from "@mui/icons-material/Height";
import { styled } from "@mui/material";
import { SubDrawer } from "./Drawer/drawer";
import Bookmark from "@mui/icons-material/Bookmark";
import SearchIcon from "@mui/icons-material/Search";

interface TopMenuProps extends PageControlProps {
  updateDocument: (doc: string) => void;
  toggleDualPage: () => void;
  toggleFitToHeight: () => void;
  documents: Document[] | null;
  fetchDocumentsError: string | null;
  uploadDoneCallback: () => void;
  drawerOpen: boolean;
  drawerWidth: number;
  setSidebarContents: (subDrawer: SubDrawer) => void;
}

const AppBar = styled(MuiAppBar, {
  shouldForwardProp: (prop) => prop !== "open" && prop !== "drawerWidth",
})<AppBarProps & { open: boolean; drawerWidth: number }>(
  ({ theme, open, drawerWidth }) => ({
    transition: theme.transitions.create(["margin", "width"], {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.leavingScreen,
    }),
    ...(open && {
      width: `calc(100% - ${drawerWidth}px)`,
      marginLeft: `${drawerWidth}px`,
      transition: theme.transitions.create(["margin", "width"], {
        easing: theme.transitions.easing.easeOut,
        duration: theme.transitions.duration.enteringScreen,
      }),
    }),
  })
);

export default function TopMenu({
  documents,
  updateDocument,
  fetchDocumentsError,
  uploadDoneCallback,
  numPages,
  setCurrentPage,
  currentPage,
  toggleDualPage,
  toggleFitToHeight,
  previousPage,
  drawerOpen,
  drawerWidth,
  setSidebarContents,
}: TopMenuProps) {
  return (
    <AppBar
      position="fixed"
      open={drawerOpen}
      drawerWidth={drawerWidth}
      sx={{ zIndex: (theme) => theme.zIndex.drawer + 1 }}
    >
      <Toolbar>
        <Box sx={{ flexGrow: 1 }}>
          <Documents
            updateDocument={updateDocument}
            documents={documents}
            fetchDocumentsError={fetchDocumentsError}
          />
        </Box>
        <IconButton onClick={() => setSidebarContents(SubDrawer.Bookmarks)}>
          <Bookmark />
        </IconButton>
        <IconButton onClick={() => setSidebarContents(SubDrawer.Search)}>
          <SearchIcon />
        </IconButton>
        {numPages != 0 && (
          <>
            <IconButton onClick={toggleFitToHeight}>
              <Height />
            </IconButton>
            <IconButton onClick={toggleDualPage}>
              <MenuBook />
            </IconButton>
            <PageControl
              {...{ currentPage, numPages, setCurrentPage, previousPage }}
            />
          </>
        )}
        <Uploader uploadDoneCallback={uploadDoneCallback} />
      </Toolbar>
    </AppBar>
  );
}
