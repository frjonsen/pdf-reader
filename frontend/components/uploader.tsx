import React from "react";
import axios from "axios";

const sendFile = (e: React.ChangeEvent<HTMLInputElement>) => {
  const formData = new FormData();
  const file = e.target?.files?.item(0);
  if (!file) {
    console.error("Got upload event with no files");
    return;
  }
  formData.append("document", file);
  console.log("Uploading file: %s", file.name);
  axios.post("/api/documents", formData, {
    headers: {
      "Content-Type": "multipart/form-data",
    },
  });
};

export function Uploader(): React.ReactElement {
  return (
    <label className="w-48 flex flex-col items-center px-4 py-6 bg-slate rounded-lg shadow-lg tracking-wide uppercase border border-blue cursor-pointer hover:bg-blue hover:text-white">
      <span className="mt-2 text-sm leading-normal">Upload document</span>
      <input type="file" className="hidden" onChange={sendFile} />
    </label>
  );
}
