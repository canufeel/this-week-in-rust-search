import React, { Component, Fragment } from 'react';
import './App.css';
import 'bootstrap/dist/css/bootstrap.min.css';
import SearchBox from './SearchBox';
import SearchResults from './SearchResults';
import { Container } from 'react-bootstrap';

/* eslint-disable no-undef */
const serverIp = SERVER_IP;
const serverPort = PORT;
const production = PRODUCTION;
const serverRoot = 'http://';
/* eslint-enable no-undef */

const buildUrl = path => production ? path : `${serverRoot}${serverIp}:${serverPort}${path}`;

const getQueryResult = async q => {
  const res = await fetch(buildUrl(`/query?query=${q}`));
  return res.json();
};

const getAllResults = async () => {
  const res = await fetch(buildUrl(`/all`));
  return res.json();
};

const timeOut = t => new Promise(resolve => setTimeout(resolve, t));

const getSlugLink = slug => buildUrl(`/slug/${slug}`);

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
      <Fragment>
        <SearchBox
          query={ this.state.query }
          queryUpdate={ (q) => this.updateQuery(q) }
        />
        <Container id="main-container">
          <SearchResults
            results={ this.state.searchResults }
            openSlugUrl={ (s) => this.openSlugUrl(s) }
          />
        </Container>
      </Fragment>
    );
  }
}

export default App;
