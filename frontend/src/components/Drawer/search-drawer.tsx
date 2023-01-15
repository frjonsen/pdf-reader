import {
  Box,
  Divider,
  IconButton,
  InputAdornment,
  List,
  ListItem,
  ListItemButton,
  ListItemText,
  OutlinedInput,
  TextField,
  Typography,
} from "@mui/material";
import { useState } from "react";
import SearchIcon from "@mui/icons-material/Search";
import axios from "axios";
import { fontSize, fontWeight } from "@mui/system";

interface SearchDrawerProps {
  setPage: (page: number) => void;
  documentId: string;
  currentPage: number;
}

interface SearchResult {
  score: number;
  page: number;
  snippet?: string;
}

export default function SearchDrawer({
  setPage,
  documentId,
}: SearchDrawerProps): JSX.Element {
  const [searchFieldContents, setSearchFieldContents] = useState("");
  const [searchResults, setSearchResults] = useState<SearchResult[]>([]);

  async function search() {
    const result = await axios.get<SearchResult[]>(
      `/api/documents/${documentId}/search?q=${searchFieldContents}`
    );
    setSearchResults(result.data);
  }

  function convertHighlight(snippet: string): JSX.Element {
    const trimmed = snippet.replace(/&#x27;/g, "'");
    const parts = [];

    let inProgress = trimmed;
    while (true) {
      const start = inProgress.indexOf("<b>");
      const end = inProgress.indexOf("</b>");
      if (start === -1) {
        parts.push(inProgress);
        break;
      }
      const before = inProgress.substring(0, start);
      parts.push(<>{before}</>);
      const highlight = inProgress.substring(start + 3, end);
      parts.push(
        <Box sx={{ display: "inline", fontWeight: "900", fontSize: "0.95rem" }}>
          {highlight}
        </Box>
      );
      inProgress = inProgress.substring(end + 4);
    }

    return (
      <Typography component="div" variant="body2">
        {parts}
      </Typography>
    );
  }

  return (
    <>
      <OutlinedInput
        endAdornment={
          <InputAdornment position="end">
            <IconButton onClick={search}>
              <SearchIcon />
            </IconButton>
          </InputAdornment>
        }
        margin="dense"
        placeholder="Search"
        value={searchFieldContents}
        onKeyDown={(e) => {
          if (e.key == "Enter") search();
        }}
        onChange={(e) => {
          e.stopPropagation();
          setSearchFieldContents(e.target.value);
        }}
      />
      <Divider />
      <List>
        {searchResults.map((s, i) => (
          <ListItem key={i}>
            <ListItemButton onClick={() => setPage(s.page)}>
              <ListItemText>
                <Typography>
                  {s.page} - {s.score}
                </Typography>
                {s.snippet && convertHighlight(s.snippet)}
              </ListItemText>
            </ListItemButton>
          </ListItem>
        ))}
      </List>
    </>
  );
}
