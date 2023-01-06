import type { NextPage } from "next";
import Head from "next/head";
import { useState, useEffect } from "react";
import Viewer from "../components/viewer";
import TopMenu from "../components/topmenu";
import { Document } from "../models";
import axios from "axios";
import { Box, Toolbar } from "@mui/material";
import Drawer, { SubDrawer } from "../components/Drawer/drawer";

const Home: NextPage = () => {
  return (
    <div>
      <Head>
        <title>PDF Reader</title>
        <meta name="description" content="Generated by create next app" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main>
        <Main />
      </main>
    </div>
  );
};

function Main() {
  const [currentDocument, setDocument] = useState<Document | null>(null);
  const [documents, setDocuments] = useState<Document[] | null>(null);
  const [documentsFetchError, setDocumentsFetchError] = useState<string | null>(
    null
  );
  const [numPages, setNumPages] = useState<number | null>(null);
  const [currentPage, setCurrentPage] = useState<number>(1);
  const [previousPage, setPreviousPage] = useState<number>(1);
  const [dualPaneMode, setDualPaneMode] = useState<boolean>(false);
  const [fitToHeight, setFitToHeight] = useState<boolean>(true);
  const [drawerMode, setDrawerMode] = useState<SubDrawer>(SubDrawer.Bookmarks);
  const [drawerWidth, setDrawerWidth] = useState(340);

  const _setCurrentPage = (page: number) => {
    setPreviousPage(currentPage);
    setCurrentPage(page);
  };

  const getLastActiveDocument = () => localStorage.getItem("lastDocument");

  const updateCurrentPage = (page: number) => {
    const movement = page - currentPage;
    let actualPage = page;
    if (Math.abs(movement) == 1) {
      actualPage = currentPage + movement * (dualPaneMode ? 2 : 1);
    }
    _setCurrentPage(actualPage);

    axios
      .patch(`/api/documents/${currentDocument!.id}`, {
        current_page: actualPage,
      })
      .catch((e: Error) => {
        console.error(e);
      });
  };

  const setCurrentDocument = (
    documentId: string,
    existingDocuments: Document[] | undefined = undefined
  ) => {
    const doc = (existingDocuments ?? documents)?.find(
      (d) => d.id === documentId
    );
    if (!doc) {
      console.error("Document not found in list of known documents");
      return;
    }
    setDocument(doc);
    _setCurrentPage(doc.current_page);
    localStorage.setItem("lastDocument", doc.id);
  };

  const setCorrectCurrentPage = (docId: string, docs: Document[]) => {
    const docFromApi = docs.find((d) => d.id === docId);
    if (!docFromApi) {
      console.warn("Didn't find currently selected document in documents list");
      return;
    }

    _setCurrentPage(docFromApi.current_page);
  };

  const updateDocuments = () => {
    axios
      .get<Document[]>("/api/documents")
      .then((docs) => {
        setDocuments(docs.data);
        const lastDocument = getLastActiveDocument();
        if (!lastDocument) return;
        setCurrentDocument(lastDocument, docs.data);
        setCorrectCurrentPage(lastDocument, docs.data);
      })
      .catch((e: Error) => {
        setDocumentsFetchError(e.message);
      });
  };
  useEffect(updateDocuments, []);

  const setNumberOfPages = (numberOfPages: number) => {
    if (numPages != numberOfPages) setNumPages(numberOfPages);
  };

  return (
    <Box>
      <TopMenu
        numPages={numPages ?? 0}
        setCurrentPage={updateCurrentPage}
        currentPage={currentPage}
        updateDocument={setCurrentDocument}
        documents={documents}
        fetchDocumentsError={documentsFetchError}
        uploadDoneCallback={updateDocuments}
        toggleFitToHeight={() => setFitToHeight(!fitToHeight)}
        toggleDualPage={() => setDualPaneMode(!dualPaneMode)}
        previousPage={previousPage}
        drawerWidth={drawerWidth}
        drawerOpen={false}
        setSidebarContents={(contents) =>
          setDrawerMode(drawerMode === contents ? SubDrawer.None : contents)
        }
      />
      {currentDocument && (
        <Drawer
          width={drawerWidth}
          subDrawer={drawerMode}
          documentId={currentDocument.id}
          setPage={updateCurrentPage}
          currentPage={currentPage}
        />
      )}
      <Box>
        <Toolbar />
        {currentDocument && (
          <Viewer
            fitToHeight={fitToHeight}
            dualPane={dualPaneMode}
            currentPage={currentPage}
            setNumPages={setNumberOfPages}
            document={currentDocument.id}
            setCurrentPage={_setCurrentPage}
            drawerOpen={drawerMode !== SubDrawer.None}
            drawerWidth={drawerWidth}
          />
        )}
      </Box>
    </Box>
  );
}

export default Home;
