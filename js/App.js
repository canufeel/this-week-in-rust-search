import React, { Component } from 'react';
import './App.css';
import SearchBox from './SearchBox';
import SearchResults from './SearchResults';

// eslint-disable-next-line no-undef
const serverPath = SERVER_URL;
const serverRoot = 'http://';

const getQueryResult = async q => {
  const res = await fetch(`${serverRoot}${serverPath}/query?query=${q}`);
  return res.json();
};

const getAllResults = async () => {
  const res = await fetch(`${serverRoot}${serverPath}/all`);
  return res.json();
};

const timeOut = t => new Promise(resolve => setTimeout(resolve, t));

const getSlugLink = slug => `${serverRoot}${serverPath}/slug/${slug}`;

class App extends Component {
  state = {
    query: '',
    searchResults: [],
    pendingPromise: null,
  };

  async openSlugUrl(slug) {
    const url = getSlugLink(slug);
    const res = await fetch(url);
    const {
      url: actualUrl,
    } = await res.json();
    window.open(actualUrl, '_blank');
  }

  updateQuery(q) {
    let pendingPromise;
    if (q.length < 3) {
      pendingPromise = null;
    } else {
      pendingPromise = new Promise(async resolve => {
        await timeOut(1000);
        const searchResults = await getQueryResult(q);
        this.setState({
          searchResults,
          pendingPromise: null,
        });
        resolve();
      });
    }
    this.setState({
      query: q,
      pendingPromise,
    })
  }

  componentDidMount() {
    this.setState({
      pendingPromise: new Promise(async resolve => {
        const searchResults = await getAllResults();
        this.setState({
          pendingPromise: null,
          searchResults,
        });
        resolve();
      }),
    });
  }

  render() {
    return (
      <div className="app-container">
        <SearchBox
          query={ this.state.query }
          queryUpdate={ (q) => this.updateQuery(q) }
        />
        <SearchResults
          results={ this.state.searchResults }
          openSlugUrl={ (s) => this.openSlugUrl(s) }
        />
      </div>
    );
  }
}

export default App;
