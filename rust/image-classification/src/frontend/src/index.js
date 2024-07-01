import { backend } from "../../declarations/backend";

document.getElementById("classify").onclick = classify;
document.getElementById("add").onclick = add;
document.getElementById("file").onchange = onImageChange;

// Calls the backend to perform image classification.
async function classify(event) {
  event.preventDefault();

  const button = event.target;
  const button2 = document.getElementById("add");
  const message = document.getElementById("message");
  const loader = document.getElementById("loader");
  const img = document.getElementById("image");
  const repl_option = document.getElementById("replicated_option");

  button.disabled = true;
  button.className = "clean-button invisible";
  button2.disabled = true;
  button2.className = "clean-button invisible";
  repl_option.className = "option invisible";
  message.innerText = "Computing...";
  loader.className = "loader";

  try {
    const [blob, scaling] = await resize(img);
    let result;
    if (document.getElementById("replicated").checked) {
      result = await backend.detect(new Uint8Array(blob));
    } else {
      result = await backend.detect_query(new Uint8Array(blob));
    }
    if (result.Ok) {
      let blob = await render(message, scaling, result.Ok);
      result = await backend.recognize(new Uint8Array(blob));
      if (result.Ok) {
        message.innerText = JSON.stringify(result.Ok);
      } else {
        throw JSON.stringify(result.Err);
      }
    } else {
      throw JSON.stringify(result.Err);
    }
  } catch (err) {
    message.innerText = "Failed to detect face: " + err.toString();
  }
  loader.className = "loader invisible";

  return false;
}

async function add(event) {
  event.preventDefault();

  const button = event.target;
  const button2 = document.getElementById("classify");
  const message = document.getElementById("message");
  const loader = document.getElementById("loader");
  const img = document.getElementById("image");
  const repl_option = document.getElementById("replicated_option");
  const label = document.getElementById("label");

  button.disabled = true;
  button.className = "clean-button invisible";
  button2.disabled = true;
  button2.className = "clean-button invisible";
  repl_option.className = "option invisible";
  message.innerText = "Computing...";
  loader.className = "loader";

  try {
    const [blob, scaling] = await resize(img);
    let result;
    if (document.getElementById("replicated").checked) {
      result = await backend.detect(new Uint8Array(blob));
    } else {
      result = await backend.detect_query(new Uint8Array(blob));
    }
    if (result.Ok) {
      let blob = await render(message, scaling, result.Ok);
      result = await backend.add(label.value, new Uint8Array(blob));
      if (result.Ok) {
        message.innerText = "Embedding" + JSON.stringify(result.Ok);
      } else {
        throw JSON.stringify(result.Err);
      }
    } else {
      throw JSON.stringify(result.Err);
    }
  } catch (err) {
    message.innerText = "Failed to detect face: " + err.toString();
  }
  loader.className = "loader invisible";

  return false;
}


// Resizes the given image to 320x240px and returns the resulting PNG blob.
async function resize(img) {
  const canvas = document.createElement("canvas");
  canvas.width = 320;
  canvas.height = 240;
  let scale = Math.min(canvas.width / img.naturalWidth, canvas.height / img.naturalHeight);
  let width = img.naturalWidth * scale;
  let height = img.naturalHeight * scale;
  let x = canvas.width / 2 - width / 2;
  let y = canvas.height / 2 - height / 2;
  const ctx = canvas.getContext("2d");
  if (ctx) {
    ctx.drawImage(img, x, y, width, height);
  }
  const img2 = document.getElementById("image2");
  img2.src = canvas.toDataURL();
  let bytes = await serialize(canvas);
  return [bytes, { scale, x, y }];
}

// Serializes the given canvas into PNG image bytes.
function serialize(canvas) {
  return new Promise((resolve) => canvas.toBlob((blob) => blob.arrayBuffer().then(resolve), "image/png", 0.9));
}

// Adds the classification results as a list to the given DOM element.
async function render(element, scaling, box) {
  const message = document.getElementById("message");
  message.textContent = "";
  const img = document.getElementById("image");
  const pos = img.getBoundingClientRect();
  const canvas = document.createElement("canvas");
  const w = img.width;
  const h = img.height;
  canvas.width = w;
  canvas.height = h;

  box.left = Math.round((box.left * 320 - scaling.x) / scaling.scale);
  box.right = Math.round((box.right * 320 - scaling.x) / scaling.scale);
  box.top = Math.round((box.top * 240 - scaling.y) / scaling.scale);
  box.bottom = Math.round((box.bottom * 240 - scaling.y) / scaling.scale);

  // const canvas2 = document.createElement("canvas");
  const canvas2 = document.getElementById("canvas123");
  canvas2.width = 140;
  canvas2.height = 140;
  const ctx2 = canvas2.getContext("2d");
  if (ctx2) {
    ctx2.drawImage(img, box.left, box.top, box.right - box.left, box.bottom - box.top, 0, 0, 140, 140);
    //ctx2.drawImage(img, box.left, box.top, 140, 140, 0, 0, 140, 140);
  }
  const img2 = document.getElementById("image2");
  img2.src = canvas2.toDataURL();

  // const ctx = canvas.getContext("2d");
  // if (ctx) {
  //   ctx.drawImage(img, 0, 0, w, h);
  //   ctx.strokeStyle = "#0f3";
  //   ctx.lineWidth = 5;
  //   ctx.beginPath();
  //   ctx.rect(box.left, box.top, box.right - box.left, box.bottom - box.top);
  //   ctx.stroke();
  // }
  // img.src = canvas.toDataURL();
  let bytes = await serialize(canvas2);

  return bytes;
}

// This function is called when the user selects a new image file.
async function onImageChange(event) {
  const button = document.getElementById("classify");
  const button2 = document.getElementById("add");
  const message = document.getElementById("message");
  const img = document.getElementById("image");
  const repl_option = document.getElementById("replicated_option");
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
  button2.disabled = false;
  button2.className = "clean-button";
  message.innerText = "";
  repl_option.className = "option"
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
