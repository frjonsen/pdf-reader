import { Document, Page, pdfjs, PDFPageProxy } from "react-pdf";
import React, { useEffect, useState } from "react";
import Box from "@mui/material/Box";
import Grid from "@mui/material/Grid";
import { PDFDocumentProxy } from "pdfjs-dist/types/src/display/api";
pdfjs.GlobalWorkerOptions.workerSrc = `//cdnjs.cloudflare.com/ajax/libs/pdf.js/${pdfjs.version}/pdf.worker.js`;

interface ViewerProps {
  document: string;
  currentPage: number;
  dualPane: boolean;
  setNumPages: (numPages: number) => void;
}
function getWindowDimensions() {
  const { innerWidth: width, innerHeight: height } = window;
  return {
    width,
    height,
  };
}

function useWindowDimensions() {
  const [windowDimensions, setWindowDimensions] = useState(
    getWindowDimensions()
  );

  useEffect(() => {
    function handleResize() {
      setWindowDimensions(getWindowDimensions());
    }

    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  return windowDimensions;
}

export default function Viewer({
  currentPage,
  document,
  dualPane,
  setNumPages,
}: ViewerProps) {
  const [pdfBuiltinWidth, setPdfBuildinWidth] = useState(300);
  const windowDimensions = useWindowDimensions();

  function onDocumentLoadSuccess(document: PDFDocumentProxy) {
    console.log("Setting numPages to ", document.numPages);
    setNumPages(document.numPages);
  }

  function onPageLoad(page: PDFPageProxy) {
    if (pdfBuiltinWidth != page.originalWidth)
      setPdfBuildinWidth(page.originalWidth);
  }

  const pageWidth =
    (windowDimensions.width - (dualPane ? 20 : 0)) / (dualPane ? 2 : 1);
  const generatePage = (pageNum: number) => {
    return (
      <Grid item md={6}>
        <Page
          onLoadSuccess={onPageLoad}
          width={pageWidth}
          pageNumber={pageNum}
        />
      </Grid>
    );
  };

  return (
    <Box id="ViewerWrapper">
      <Document
        file={`/api/documents/${document}`}
        onLoadSuccess={onDocumentLoadSuccess}
      >
        <Grid container spacing={0}>
          {generatePage(currentPage)}
          {dualPane && generatePage(currentPage + 1)}
        </Grid>
      </Document>
    </Box>
  );
}
