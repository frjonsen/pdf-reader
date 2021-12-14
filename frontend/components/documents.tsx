import { Document } from "../models";
import { useEffect } from "react";

interface DocumentsProps {
  updateDocument: (doc: string) => void;
}

export function Documents({
  updateDocument,
}: DocumentsProps): React.ReactElement {
  return <a onClick={() => updateDocument("Some kind of id")}>Hello</a>;
}
