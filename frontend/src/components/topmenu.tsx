import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Box from "@mui/material/Box";
import { Documents } from "./documents";
import { Uploader } from "./uploader";
import { Document } from "../models";
import PageControl, { PageControlProps } from "./pagecontrol";

interface TopMenuProps extends PageControlProps {
  updateDocument: (doc: string) => void;
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
          <PageControl
            {...{ currentPage, numPages, setCurrentPage }}
          ></PageControl>
        )}
        <Uploader uploadDoneCallback={uploadDoneCallback} />
      </Toolbar>
    </AppBar>
  );
}
