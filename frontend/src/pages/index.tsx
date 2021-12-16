import type { NextPage } from "next";
import Head from "next/head";
import { useState, useEffect } from "react";
import { Documents } from "../components/documents";
import { Uploader } from "../components/uploader";
import TopMenu from "../components/topmenu";
import { Document } from "../models";
import axios from "axios";

const Home: NextPage = () => {
  return (
    <div>
      <Head>
        <title>Create Next App</title>
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
  const [currentDocument, setDocument] = useState<string | null>(null);
  const [documents, setDocuments] = useState<Document[] | null>(null);
  const [documentsFetchError, setDocumentsFetchError] = useState<string | null>(
    null
  );

  const updateDocuments = () => {
    axios
      .get<Document[]>("/api/documents")
      .then((docs) => {
        setDocuments(docs.data);
      })
      .catch((e) => {
        setDocumentsFetchError(e);
      });
  };
  useEffect(updateDocuments, []);

  let text: string;
  if (currentDocument === null) {
    text = "No document selected";
  } else {
    text = `Current document is ${currentDocument}`;
  }

  return (
    <>
      <TopMenu
        updateDocument={setDocument}
        documents={documents}
        fetchDocumentsError={documentsFetchError}
        uploadDoneCallback={updateDocuments}
      />
      <p>{text}</p>
    </>
  );
}

export default Home;
