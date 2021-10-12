import React from 'react';
import logo from './logo.svg';
import './App.css';

interface AppState {
  pdfs: Array<String>
}

const Pdf = (props: { pdfs: Array<String> }) => <ul id="pdfList">{props.pdfs.map(f => <li>{f}</li>)}</ul>;

class App extends React.Component<{}, AppState> {
  state: AppState = {
    pdfs: ["1", "2", "3"]
  };

  doDifferentState = () => {
    this.setState({
      pdfs: ["2", "3", "4"]
    });
  }

  async componentDidMount() {
    const response = await fetch("/api/documents");
    const documents = await response.json();
    this.setState({ pdfs: documents });
  }

  render() {
    return (
      <div className="App">
        <Pdf pdfs={this.state.pdfs} />
      </div>
    );
  }
}

export default App;
