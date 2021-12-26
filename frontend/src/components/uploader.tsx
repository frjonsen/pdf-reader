import React from "react";
import axios from "axios";
import Button from "@mui/material/Button";
import { styled } from "@mui/material/styles";

const sendFile = (
  e: React.ChangeEvent<HTMLInputElement>,
  uploadDoneCallback: () => void
) => {
  const formData = new FormData();
  const file = e.target?.files?.item(0);
  if (!file) {
    console.error("Got upload event with no files");
    return;
  }
  formData.append("document", file);
  console.log("Uploading file: %s", file.name);
  axios
    .post("/api/documents", formData, {
      headers: {
        "Content-Type": "multipart/form-data",
      },
    })
    .then(uploadDoneCallback);
};

interface UploadProps {
  uploadDoneCallback: () => void;
}

const Input = styled("input")({
  display: "none",
});

export function Uploader({
  uploadDoneCallback,
}: UploadProps): React.ReactElement {
  return (
    <label htmlFor="contained-button-file">
      <Input
        accept="application/pdf"
        id="contained-button-file"
        type="file"
        onChange={(e) => sendFile(e, uploadDoneCallback)}
      />
      <Button variant="contained" component="span">
        Upload
      </Button>
    </label>
  );
}
