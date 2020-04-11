import React from 'react';
import { Col, Container, Jumbotron, Row } from 'react-bootstrap';

const SearchResults = ({
  results,
  openSlugUrl,
}) => (
  <Jumbotron fluid id="results">
    <Container>
      { results.map(({
        slug,
        text,
        date,
      }) => (
        <Row key={slug}>
          <Col md={2}>
            <p className="date-field smaller-bottom-padding">
            { date }
            </p>
          </Col>
          <Col md={10} onClick={ () => openSlugUrl(slug) }>
            <p className="results-item smaller-bottom-padding">
            { text }
            </p>
          </Col>
        </Row>
      ))}
    </Container>
  </Jumbotron>
);

export default SearchResults;
