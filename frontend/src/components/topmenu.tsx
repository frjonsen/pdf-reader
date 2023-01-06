import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Box from "@mui/material/Box";
import { Documents } from "./documents";
import { Uploader } from "./uploader";
import { Document } from "../models";
import PageControl, { PageControlProps } from "./pagecontrol";
import IconButton from "@mui/material/IconButton";
import MenuBook from "@mui/icons-material/MenuBook";
import Height from "@mui/icons-material/Height";

interface TopMenuProps extends PageControlProps {
  updateDocument: (doc: string) => void;
  toggleDualPage: () => void;
  toggleFitToHeight: () => void;
  documents: Document[] | null;
  fetchDocumentsError: string | null;
  uploadDoneCallback: () => void;
}

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
}: TopMenuProps) {
  return (
    <AppBar position="static">
      <Toolbar>
        <Box sx={{ flexGrow: 1 }}>
          <Documents
            updateDocument={updateDocument}
            documents={documents}
            fetchDocumentsError={fetchDocumentsError}
          />
        </Box>
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
