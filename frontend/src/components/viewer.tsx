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
  fitToHeight: boolean;
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

export default function Viewer(props: ViewerProps) {
  const [pdfBuiltinWidth, setPdfBuildinWidth] = useState(300);
  const windowDimensions = useWindowDimensions();

  function onDocumentLoadSuccess(_document: PDFDocumentProxy) {
    props.setNumPages(_document.numPages);
  }

  function onPageLoad(page: PDFPageProxy) {
    if (pdfBuiltinWidth != page.originalWidth)
      setPdfBuildinWidth(page.originalWidth);

    const textLayers = document.querySelectorAll(
      ".react-pdf__Page__textContent"
    );
    textLayers.forEach((layer: any) => {
      const { style } = layer;
      style.top = "0";
      style.left = "0";
      style.transform = "";
      style.lineHeight = "1.0";
    });
  }

  const pageWidth = windowDimensions.width - (props.dualPane ? 20 : 0);
  const splitPageWidth = pageWidth / (props.dualPane ? 2 : 1);
  const generatePage = (pageNum: number) => {
    return (
      <Grid item>
        <Page
          onLoadSuccess={onPageLoad}
          width={props.fitToHeight ? undefined : splitPageWidth}
          height={props.fitToHeight ? windowDimensions.height : undefined}
          pageNumber={pageNum}
        />
      </Grid>
    );
  };

  return (
    <Box id="ViewerWrapper">
      <Document
        file={`/api/documents/${props.document}`}
        onLoadSuccess={onDocumentLoadSuccess}
      >
        <Grid container spacing={1} justifyContent="center">
          {generatePage(props.currentPage)}
          {props.dualPane && generatePage(props.currentPage + 1)}
        </Grid>
      </Document>
    </Box>
  );
}
