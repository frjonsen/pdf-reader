import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Box from "@mui/material/Box";
import { Documents } from "./documents";
import { Uploader } from "./uploader";
import { Document } from "../models";

interface TopMenuProps {
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
        <Uploader uploadDoneCallback={uploadDoneCallback} />
      </Toolbar>
    </AppBar>
  );
}
