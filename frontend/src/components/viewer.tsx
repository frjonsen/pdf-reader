import { Document, Page, pdfjs, PDFPageProxy } from "react-pdf";
import React, { useEffect, useState } from "react";
import Box from "@mui/material/Box";
import Grid from "@mui/material/Grid";
pdfjs.GlobalWorkerOptions.workerSrc = `//cdnjs.cloudflare.com/ajax/libs/pdf.js/${pdfjs.version}/pdf.worker.js`;

interface ViewerProps {
  document: string;
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

export default function Viewer({ document }: ViewerProps) {
  const [pageNumber, setPageNumber] = useState(5);
  const [pdfBuiltinWidth, setPdfBuildinWidth] = useState(300);
  const [numPages, setNumPages] = useState(0);
  const windowDimensions = useWindowDimensions();
  const calcZoom = (pdfWidth: number, windowWidth: number) =>
    windowWidth / (pdfWidth * 2);
  const [zoom, setZoom] = useState(
    calcZoom(pdfBuiltinWidth, windowDimensions.width)
  );

  function onDocumentLoadSuccess(numPages: PDFPageProxy) {
    setNumPages(numPages.numPages);
    setPdfBuildinWidth(numPages.originalWidth);
    setZoom(calcZoom(numPages.originalHeight, windowDimensions.width));
  }

  console.log(zoom);

  return (
    <Box id="ViewerWrapper">
      <Document file={`/api/documents/${document}`}>
        onLoadSuccess={onDocumentLoadSuccess}
        <Grid container spacing={0}>
          <Grid item md={6}>
            <Page scale={zoom} pageNumber={pageNumber} />
          </Grid>
          <Grid item md={6}>
            <Page
              scale={zoom}
              pageNumber={pageNumber + 1}
              onLoadSuccess={onDocumentLoadSuccess}
            />
          </Grid>
        </Grid>
        <p>
          Page {pageNumber} of {numPages}
        </p>
      </Document>
    </Box>
  );
}
