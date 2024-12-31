/*
 * Copyright (C) 2024 Oliver Pirger <0x4f48@proton.me>
 *
 * This file is part of Warehouse.
 *
 * Warehouse is free software: you can redistribute it and/or modify it under the terms of
 * the GNU Affero General Public License as published by the Free Software Foundation,
 * version 3 of the License only.
 *
 * Warehouse is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with Warehouse. If not, see <https://www.gnu.org/licenses/>.
 */

let key = "";
const value = `; ${document.cookie}`;
const parts = value.split(`; key=`);
if (parts.length === 2) key = parts.pop().split(";").shift();

let keyPage = document.querySelector("main#key");
let uploadPage = document.querySelector("main#upload");
let downloadPage = document.querySelector("main#download");
let deletePage = document.querySelector("main#delete");

let keyBtn = document.querySelector("button#key");
let uploadBtn = document.querySelector("button#upload");
let downloadBtn = document.querySelector("button#download");
let deleteBtn = document.querySelector("button#delete");

keyBtn.disabled = true;

function hideAll() {
  keyPage.classList.remove("active");
  uploadPage.classList.remove("active");
  downloadPage.classList.remove("active");
  deletePage.classList.remove("active");
}

function enableAll() {
  keyBtn.disabled = false;
  uploadBtn.disabled = false;
  downloadBtn.disabled = false;
  deleteBtn.disabled = false;
}

keyBtn.addEventListener("click", () => {
  enableAll();
  hideAll();

  keyBtn.disabled = true;
  keyPage.classList.add("active");
});
uploadBtn.addEventListener("click", () => {
  enableAll();
  hideAll();

  uploadBtn.disabled = true;
  uploadPage.classList.add("active");
});
downloadBtn.addEventListener("click", () => {
  enableAll();
  hideAll();

  downloadBtn.disabled = true;
  downloadPage.classList.add("active");
});
deleteBtn.addEventListener("click", () => {
  enableAll();
  hideAll();

  deleteBtn.disabled = true;
  deletePage.classList.add("active");
});

// KEY PAGE
let keyForm = document.querySelector("form#key");
let keySubmit = document.querySelector("form#key button");
keyForm.addEventListener("submit", (event) => {
  event.preventDefault();

  const formData = new FormData(keyForm);
  const newkey = formData.get("key");

  document.cookie = `key=${newkey}; SameSite=Strict`;
  key = newkey;
  keySubmit.innerText = "Saved!";
  setTimeout(() => {
    keySubmit.innerText = "Save";
  }, 3000);
});

// UPLOAD PAGE
let form = document.querySelector("form#upload");
let submitButton = document.getElementById("submit");
form.addEventListener("submit", async (event) => {
  event.preventDefault();

  submitButton.style.cursor = "wait";
  submitButton.disabled = true;

  const formData = new FormData(form);
  await fetch("/", {
    method: "POST",
    body: formData,
    headers: {
      Authorization: `Basic ${key}`,
    },
  }).then(async (response) => {
    submitButton.style.cursor = "default";
    submitButton.disabled = false;

    let responseField = document.getElementById("response");
    responseField.innerText = await response.text();
  });
});

let dropArea = document.getElementById("drop");
let filenameContainer = document.querySelector(".filename");
let input = document.getElementById("file");
["dragenter", "dragover", "dragleave", "drop"].forEach((eventName) => {
  dropArea.addEventListener(eventName, (event) => {
    event.preventDefault();
    event.stopPropagation();
  });
});
["dragenter", "dragover"].forEach((event) => {
  dropArea.addEventListener(event, () => {
    dropArea.classList.add("mouseover");
    filenameContainer.classList.add("mouseover");
  });
});
["dragleave", "drop"].forEach((event) => {
  dropArea.addEventListener(event, () => {
    dropArea.classList.remove("mouseover");
    filenameContainer.classList.remove("mouseover");
  });
});
let filename = document.getElementById("filename");
dropArea.addEventListener("drop", (event) => {
  const data = event.dataTransfer;
  const files = data.files;
  const dataTransfer = new DataTransfer();
  dataTransfer.items.add(files[0]);
  input.files = dataTransfer.files;
  filenameContainer.style.display = "flex";
  filename.innerText = input.files[0].name;
});
filenameContainer.style.display = "none";
input.addEventListener("change", () => {
  filenameContainer.style.display = "flex";
  filename.innerText = input.files[0].name;
});

// DOWNLOAD PAGE
const downloadForm = document.querySelector("form#download");
const downloadButton = document.querySelector("#download button");
downloadForm.addEventListener("submit", async (event) => {
  event.preventDefault();
  downloadButton.disabled = true;
  const formData = new FormData(downloadForm);
  const hash = formData.get("hash");
  await fetch(hash, {
    headers: {
      Authorization: `Basic ${key}`,
    },
  }).then(async (response) => {
    downloadButton.disabled = false;
    const errorField = document.getElementById("downloadError");
    if (response.ok == true) {
      const blob = await response.blob();
      const url = window.URL.createObjectURL(blob);
      const link = document.createElement("a");
      link.href = url;
      link.download = response.headers
        .get("Content-Disposition")
        .split("filename=")[1]
        .replaceAll('"', "");
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      window.URL.revokeObjectURL(url);

      errorField.innerText = "";
    } else {
      errorField.innerText = `${response.status}: ${response.statusText}`;
    }
  });
});

// DELETE PAGE
const deleteForm = document.querySelector("form#delete");
const deleteButton = document.querySelector("#delete button");
deleteForm.addEventListener("submit", async (event) => {
  event.preventDefault();
  deleteButton.disabled = true;
  const formData = new FormData(deleteForm);
  const hash = formData.get("hash");
  const errorField = document.getElementById("deleteError");
  await fetch(hash, {
    headers: {
      Authorization: `Basic ${key}`,
    },
    method: "DELETE",
  }).then((response) => {
    errorField.innerText = "";
    deleteButton.disabled = false;
    if (response.ok == true) {
      deleteButton.innerText = "Deleted!";
      setTimeout(() => {
        deleteButton.innerText = "Delete";
      }, 3000);
    } else {
      errorField.innerText = `${response.status}: ${response.statusText}`;
    }
  });
});
