import React from 'react';

const SearchResults = ({
  results,
  openSlugUrl,
}) => (
  <div className="results-box">
    { results.map(({
      slug,
      text,
      date,
    }) => (
      <div key={slug} className="results-row">
        <p onClick={ () => openSlugUrl(slug) }>{ date } : { text }</p>
      </div>
    ))}
  </div>
);

export default SearchResults;
