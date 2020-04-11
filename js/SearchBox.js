import React from 'react';

const SearchBox = ({
  query,
  queryUpdate,
}) => (
  <div className="search-box">
    <input
      value={query}
      onChange={ (e) => queryUpdate(e.target.value) }
    />
  </div>
);

export default SearchBox;