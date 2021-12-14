import { Document } from "../models";
import { useEffect, useState } from "react";
import axios from "axios";

interface DocumentsProps {
  updateDocument: (doc: string) => void;
}

interface DocumentProps {
  document: Document;
}

function DocumentLink({ document }: DocumentProps) {
  return <a>{document.name}</a>;
}

export function Documents({
  updateDocument,
}: DocumentsProps): React.ReactElement {
  const [documents, setDocuments] = useState<Document[] | undefined>(undefined);
  const [error, setError] = useState<string | undefined>(undefined);

  useEffect(() => {
    axios
      .get<Document[]>("/api/documents")
      .then((docs) => {
        setDocuments(docs.data);
      })
      .catch((e) => {
        setError(e);
      });
  }, []);

  if (error !== undefined) {
    return <div>Error while fetching documents {error}</div>;
  }

  if (documents === undefined) {
    return <div>Loading...</div>;
  }
  if (documents.length === 0) {
    return <div>No documents found</div>;
  }

  return (
    <div>
      {documents.map((d) => (
        <a key={d.id} onClick={() => updateDocument(d.id)}>
          {d.name}
        </a>
      ))}
    </div>
  );
}
