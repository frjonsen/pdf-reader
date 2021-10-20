import { Document, Page } from 'react-pdf/dist/esm/entry.webpack';
import React from 'react';
import logo from './logo.svg';
import './App.css';

interface AppState {
  pdfs: Array<String>
  currentDocumentTotalPages: number
}

const Pdf = (props: { pdfs: Array<String> }) => <ul id="pdfList">{props.pdfs.map(f => <li>{f}</li>)}</ul>;

class App extends React.Component<{}, AppState> {
  state: AppState = {
    pdfs: ["1", "2", "3"],
    currentDocumentTotalPages: 0
  };

  doDifferentState = () => {
    this.setState({
      pdfs: ["2", "3", "4"],
    });
  }

  async componentDidMount() {
    const response = await fetch("/api/documents");
    const documents = await response.json();
    this.setState({ pdfs: documents });
  }

  onDocumentLoadSuccess = ({ numPages }: { numPages: number }) => {
    this.setState({
      currentDocumentTotalPages: numPages
    });
  }

  render() {
    return (
      <div className="App">
        <Pdf pdfs={this.state.pdfs} />
        <div id="documentViewer">

          <Document file="/api/documents/1" onLoadSuccess={this.onDocumentLoadSuccess}>
            <Page pageNumber={1} />
            <Page pageNumber={2} />
          </Document>
        </div>
        <p>{this.state.currentDocumentTotalPages} pages in this document</p>
      </div>
    );
  }
}

export default App;
