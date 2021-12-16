import { Document, Page, pdfjs, PDFPageProxy } from "react-pdf";
import { useState } from "react";
import Box from "@mui/material/Box";
pdfjs.GlobalWorkerOptions.workerSrc = `//cdnjs.cloudflare.com/ajax/libs/pdf.js/${pdfjs.version}/pdf.worker.js`;

interface ViewerProps {
  document: string;
}

export default function Viewer({ document }: ViewerProps) {
  const [pageNumber, setPageNumber] = useState(1);
  const [numPages, setNumPages] = useState(0);

  function onDocumentLoadSuccess(numPages: PDFPageProxy) {
    setNumPages(numPages.pageNumber);
  }

  return (
    <Box id="ViewerWrapper" sx={{ width: 1 }}>
      <Document file={`/api/documents/${document}`}>
        <Page
          pageNumber={pageNumber}
          renderTextLayer={false}
          onLoadSuccess={onDocumentLoadSuccess}
        />
        <p>
          Page {pageNumber} of {numPages}
        </p>
      </Document>
    </Box>
  );
}
