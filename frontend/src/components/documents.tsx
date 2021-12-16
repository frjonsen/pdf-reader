import { Document } from "../models";
import NativeSelect from "@mui/material/NativeSelect";
import { useState } from "react";

interface DocumentsProps {
  updateDocument: (doc: string) => void;
  documents: Document[] | null;
  fetchDocumentsError: string | null;
}

interface DocumentProps {
  document: Document;
}

function DocumentLink({ document }: DocumentProps) {
  return <a>{document.name}</a>;
}

export function Documents({
  updateDocument,
  documents,
  fetchDocumentsError,
}: DocumentsProps): React.ReactElement {
  const [hasBeenSet, setHasBeenSet] = useState(false);

  if (fetchDocumentsError !== null) {
    return <div>Error while fetching documents {fetchDocumentsError}</div>;
  }

  if (documents === null) {
    return <div>Loading...</div>;
  }
  if (documents.length === 0) {
    return <div>No documents found</div>;
  }

  return (
    <NativeSelect
      onChange={(e) => {
        setHasBeenSet(true);
        updateDocument(e.target.value);
      }}
      sx={{ width: 0.99 }}
    >
      {!hasBeenSet && <option value="">None</option>}
      {documents.map((d) => (
        <option key={d.id} value={d.id}>
          {d.name}
        </option>
      ))}
    </NativeSelect>
  );
}
