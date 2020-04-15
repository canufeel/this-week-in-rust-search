import React from 'react';
import { makeStyles } from '@material-ui/core/styles';
import Card from '@material-ui/core/Card';
import CardContent from '@material-ui/core/CardContent';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import ListItemIcon from '@material-ui/core/ListItemIcon';
import ListItemText from '@material-ui/core/ListItemText';
import AnnouncementIcon from '@material-ui/icons/Announcement';

const useStyles = makeStyles({
  root: {
    minWidth: 275,
  },
  title: {
    fontSize: 14,
  },
});

const SearchResults = ({
  results,
  openSlugUrl,
}) => {
  const classes = useStyles();
  return (
    <Card className={classes.root}>
      <CardContent>
        <List component="nav" aria-label="main mailbox folders">
          { results.map(({
            slug,
            text,
            date,
          }) => (
            <ListItem
              key={slug}
              button
              onClick={ () => openSlugUrl(slug)}
            >
              <ListItemIcon>
                <AnnouncementIcon />
              </ListItemIcon>
              <ListItemText
                primary={ text }
                secondary={ date }
              />
            </ListItem>
          ))}
        </List>
      </CardContent>
    </Card>
  )
};

export default SearchResults;
