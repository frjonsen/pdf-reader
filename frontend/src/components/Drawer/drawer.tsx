import MuiDrawer from "@mui/material/Drawer";
import Toolbar from "@mui/material/Toolbar";
import BookmarkDrawer from "./bookmark-drawer";
import SearchDrawer from "./search-drawer";

export enum SubDrawer {
  Bookmarks,
  Search,
  None,
}

interface DrawerProps {
  subDrawer: SubDrawer;
  width: number;
  documentId: string;
  currentPage: number;
  setPage: (page: number) => void;
}

const renderBookmarksComponent = ({
  documentId,
  setPage,
  currentPage,
}: DrawerProps) => {
  return (
    <BookmarkDrawer
      documentId={documentId}
      setPage={setPage}
      currentPage={currentPage}
    />
  );
};

const renderSearchComponent = ({ documentId, setPage }: DrawerProps) => {
  return (
    <SearchDrawer documentId={documentId} setPage={setPage} currentPage={0} />
  );
};

export default function Drawer(props: DrawerProps) {
  let subDrawerComponent = <></>;

  if (props.subDrawer === SubDrawer.Bookmarks) {
    subDrawerComponent = renderBookmarksComponent(props);
  } else if (props.subDrawer === SubDrawer.Search) {
    subDrawerComponent = renderSearchComponent(props);
  }

  return (
    <MuiDrawer
      variant="persistent"
      open={props.subDrawer !== SubDrawer.None}
      sx={{
        width: props.width,
        flexShrink: 0,
        "& .MuiDrawer-paper": {
          width: props.width,
          boxSizing: "border-box",
        },
      }}
    >
      <Toolbar />
      {subDrawerComponent}
    </MuiDrawer>
  );
}
