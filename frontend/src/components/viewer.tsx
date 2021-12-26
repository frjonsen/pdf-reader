import { Document, Page, pdfjs, PDFPageProxy } from "react-pdf";
import React, { useEffect, useState } from "react";
import Box from "@mui/material/Box";
import Grid from "@mui/material/Grid";
import { PDFDocumentProxy } from "pdfjs-dist/types/src/display/api";
pdfjs.GlobalWorkerOptions.workerSrc = `//cdnjs.cloudflare.com/ajax/libs/pdf.js/${pdfjs.version}/pdf.worker.js`;

interface ViewerProps {
  document: string;
  currentPage: number;
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
  setNumPages,
}: ViewerProps) {
  const [pdfBuiltinWidth, setPdfBuildinWidth] = useState(300);
  const windowDimensions = useWindowDimensions();
  const calcZoom = (pdfWidth: number, windowWidth: number) =>
    windowWidth / pdfWidth;
  const [zoom, setZoom] = useState(
    calcZoom(pdfBuiltinWidth, windowDimensions.width)
  );

  function onDocumentLoadSuccess(document: PDFDocumentProxy) {
    console.log("Setting numPages to ", document.numPages);
    setNumPages(document.numPages);
  }

  function onPageLoad(page: PDFPageProxy) {
    if (pdfBuiltinWidth != page.originalWidth)
      setPdfBuildinWidth(page.originalWidth);
    const newZoom = calcZoom(page.originalWidth, windowDimensions.width);
    if (zoom != newZoom)
      setZoom(calcZoom(page.originalWidth, windowDimensions.width));
  }

  return (
    <Box id="ViewerWrapper">
      <Document
        file={`/api/documents/${document}`}
        onLoadSuccess={onDocumentLoadSuccess}
      >
        <Grid container spacing={0}>
          <Grid item md={6}>
            <Page
              onLoadSuccess={onPageLoad}
              scale={zoom}
              pageNumber={currentPage}
            />
          </Grid>
        </Grid>
      </Document>
    </Box>
  );
}
