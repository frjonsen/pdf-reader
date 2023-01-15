import Stack from "@mui/material/Stack";
import IconButton from "@mui/material/IconButton";
import ArrowForward from "@mui/icons-material/ArrowForward";
import ArrowBack from "@mui/icons-material/ArrowBack";
import ReplayIcon from "@mui/icons-material/Replay";
import { useEffect, useState } from "react";
import TextField from "@mui/material/TextField";
import React from "react";
import Typography from "@mui/material/Typography";

export interface PageControlProps {
  numPages: number;
  setCurrentPage: (currentPage: number) => void;
  currentPage: number;
  previousPage: number;
}

export default function PageControl({
  numPages,
  setCurrentPage,
  currentPage,
  previousPage,
}: PageControlProps) {
  const inputRef: React.Ref<HTMLInputElement> = React.createRef();
  const updateCurrentPage = (currentPage: number) => {
    setCurrentPage(currentPage);
  };

  const movePage = (direction: number) => {
    if (direction < 0 && currentPage > 1) {
      updateCurrentPage(currentPage - 1);
    }
    if (direction > 0 && currentPage < numPages) {
      updateCurrentPage(currentPage + 1);
    }
    if (inputRef.current) {
      inputRef.current.value = "";
    }
  };

  useEffect(() => {
    const handleGlobalKey = (event: KeyboardEvent) => {
      if (event.key == "ArrowDown") {
        movePage(1);
      }
      if (event.key == "ArrowUp") {
        movePage(-1);
      }
    };
    window.addEventListener("keydown", handleGlobalKey, false);
    return () => window.removeEventListener("keydown", handleGlobalKey, false);
  });

  const pageChangedByNumber = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key != "Enter") return;

    const newValue = (e.target as HTMLInputElement).value;
    if (!newValue.match(/^[1-9]\d*$/)) {
      console.error("Invalid value: ", newValue);
      return;
    }
    const parsedValue = Number.parseInt(newValue, 10);
    if (parsedValue != currentPage) {
      if (inputRef.current) {
        inputRef.current.value = "";
      }
      updateCurrentPage(parsedValue);
    }
  };

  return (
    <Stack alignItems="center" direction="row">
      <IconButton onClick={() => updateCurrentPage(previousPage)}>
        <ReplayIcon />
      </IconButton>
      <IconButton onClick={() => movePage(-1)}>
        <ArrowBack />
      </IconButton>
      <TextField
        inputRef={inputRef}
        sx={{ maxWidth: 70 }}
        inputMode="numeric"
        onKeyUp={pageChangedByNumber}
        placeholder={currentPage.toString()}
      />
      <Typography sx={{ paddingLeft: 1 }}>/ {numPages}</Typography>
      <IconButton onClick={() => movePage(1)}>
        <ArrowForward />
      </IconButton>
    </Stack>
  );
}
