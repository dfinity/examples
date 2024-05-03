import { backend } from "../../declarations/backend";

document.getElementById("classify").onclick = classify;
document.getElementById("file").onchange = onImageChange;

// Calls the backend to perform image classification.
async function classify(event) {
  event.preventDefault();

  const button = event.target;
  const message = document.getElementById("message");
  const loader = document.getElementById("loader");
  const img = document.getElementById("image");

  button.disabled = true;
  button.className = "clean-button invisible";
  message.innerText = "Computing...";
  loader.className = "loader";

  try {
    const blob = await resize(img);
    const result = await backend.classify(new Uint8Array(blob));
    if (result.Ok) {
      render(message, result.Ok);
    } else {
      throw result.Err;
    }
  } catch (err) {
    message.innerText = "Failed to classify image: " + JSON.stringify(err);
  }
  loader.className = "loader invisible";

  return false;
}

// Resizes the given image to 224x224px and returns the resulting PNG blob.
async function resize(img) {
  const canvas = document.createElement("canvas");
  canvas.width = 224;
  canvas.height = 224;
  let scale = Math.max(canvas.width / img.naturalWidth, canvas.height / img.naturalHeight);
  let width = img.naturalWidth * scale;
  let height = img.naturalHeight * scale;
  let x = canvas.width / 2 - width / 2;
  let y = canvas.height / 2 - height / 2;
  const ctx = canvas.getContext("2d");
  if (ctx) {
    ctx.drawImage(img, x, y, width, height);
  }
  let bytes = await serialize(canvas);
  return bytes;
}

// Serializes the given canvas into PNG image bytes.
function serialize(canvas) {
  return new Promise((resolve) => canvas.toBlob((blob) => blob.arrayBuffer().then(resolve), "image/png", 0.9));
}

// Adds the classification results as a list to the given DOM element.
function render(element, classification) {
  element.innerText = "Results:";
  let ul = document.createElement("ul");
  for (let item of classification) {
    let li = document.createElement("li");
    let b = document.createElement("b");
    b.innerText = item.label.toLowerCase();
    let t = document.createTextNode("[score " + Math.round(item.score * 10) / 10 + "]");
    li.appendChild(b);
    li.appendChild(t);
    ul.appendChild(li)
  }
  element.appendChild(ul);
}

// This function is called when the user selects a new image file.
async function onImageChange(event) {
  const button = document.getElementById("classify");
  const message = document.getElementById("message");
  const img = document.getElementById("image");
  try {
    const file = event.target.files[0];
    const url = await toDataURL(file);
    img.src = url;
    img.width = 600;
    img.className = "image";
  } catch (err) {
    message.innerText = "Failed to select image: " + err.toString();
  }
  button.disabled = false;
  button.className = "clean-button";
  message.innerText = "";
  return false;
}

// Converts the given blob into a data url such that it can be assigned as a
// target of a link of as an image source.
function toDataURL(blob) {
  return new Promise((resolve, _) => {
    const fileReader = new FileReader();
    fileReader.readAsDataURL(blob);
    fileReader.onloadend = function () {
      resolve(fileReader.result);
    }
  });
}
