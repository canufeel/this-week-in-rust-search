import React from 'react';
import { Form, FormControl, Navbar } from 'react-bootstrap';

const SearchBox = ({
  query,
  queryUpdate,
}) => (
  <Navbar bg="dark" variant="dark" expand="lg" fixed="top">
    <Navbar.Brand href="#home">This week in Rust search</Navbar.Brand>
    <Form inline>
      <FormControl
        id="search-input"
        className="offset-md-4"
        placeholder="Enter query"
        value={query}
        onChange={ (e) => queryUpdate(e.target.value) }
      />
    </Form>
  </Navbar>
);

export default SearchBox;