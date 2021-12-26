import Stack from "@mui/material/Stack";
import IconButton from "@mui/material/IconButton";
import ArrowForward from "@mui/icons-material/ArrowForward";
import ArrowBack from "@mui/icons-material/ArrowBack";
import { useState } from "react";
import TextField from "@mui/material/TextField";
import React from "react";
import { Typography } from "@mui/material";

export interface PageControlProps {
  numPages: number;
  setCurrentPage: (currentPage: number) => void;
  currentPage: number;
}

export default function PageControl({
  numPages,
  setCurrentPage,
  currentPage,
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
