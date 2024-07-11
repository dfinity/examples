import { backend } from "../../declarations/backend";

window.onload = async () => {
  elem("recognize").onclick = recognize;
  elem("store").onclick = store;
  elem("file").onchange = load_local_image;
  elem("canvas").onclick = restart;
  elem("restart").onclick = restart;
  elem("video").oncanplay = () => {
    show("video");
    hide("image");
    hide("canvas");
  }

  navigator.mediaDevices
    .getUserMedia({ video: true, audio: false })
    .then((stream) => {
      const video = elem("video");
      video.srcObject = stream;
      video.play();
      show("buttons");
    })
    .catch((err) => {
      show("image");
      hide("buttons");
      hide("video");
      hide("canvas");
      console.error(`An error occurred: ${err}`);
      message("Couldn't start camera, but you can upload photos.")
    });
}

// Returns a DOM element that is currently visible and contains an image.
function select_visible_element() {
  const video = elem("video");
  const image = elem("image");
  const canvas = elem("canvas");
  if (!video.className.includes("invisible")) {
    return [video, video.videoWidth, video.videoHeight];
  } else if (!image.className.includes("invisible")) {
    return [image, image.width, image.height];
  } else {
    return [canvas, canvas.width, canvas.height];
  }
}

// Captures the image from the camera stream or from the image element or from
// the canvas element depending on which one of them is visible.
//
// It also scales the image down to 320x240px such that it can be submitted to
// the backend for face detection.
async function capture_image() {
  let [image, width, height] = select_visible_element();

  const canvas = elem("canvas");
  canvas.width = width
  canvas.height = height;
  const context = canvas.getContext("2d");
  context.drawImage(image, 0, 0, width, height);

  const resized = document.createElement("canvas");
  resized.width = 320;
  resized.height = 240;
  let scale = Math.min(resized.width / canvas.width, resized.height / canvas.height);
  width = canvas.width * scale;
  height = canvas.height * scale;
  let x = resized.width / 2 - width / 2;
  let y = resized.height / 2 - height / 2;
  const ctx = resized.getContext("2d");
  if (ctx) {
    ctx.drawImage(canvas, x, y, width, height);
  }
  let bytes = await serialize(resized);

  if (video.srcObject) {
    video.srcObject.getTracks().forEach((track) => track.stop());
  }

  hide("video");
  hide("image");
  show("canvas")
  return [bytes, { scale, x, y }];
}

// Renders the bounding box around the face and also returns a small image of
// the face that can be submitted to the backend for recognition.
async function render(scaling, box) {
  box.left = Math.round((box.left * 320 - scaling.x) / scaling.scale);
  box.right = Math.round((box.right * 320 - scaling.x) / scaling.scale);
  box.top = Math.round((box.top * 240 - scaling.y) / scaling.scale);
  box.bottom = Math.round((box.bottom * 240 - scaling.y) / scaling.scale);

  const canvas = elem("canvas");

  const small = document.createElement("canvas");
  small.width = 160;
  small.height = 160;
  const ctx2 = small.getContext("2d");
  if (ctx2) {
    ctx2.drawImage(canvas, box.left, box.top, box.right - box.left, box.bottom - box.top, 0, 0, 140, 140);
  }
  let bytes = await serialize(small);

  const ctx = canvas.getContext("2d");
  if (ctx) {
    ctx.strokeStyle = "#0f3";
    ctx.lineWidth = 5;
    ctx.beginPath();
    ctx.rect(box.left, box.top, box.right - box.left, box.bottom - box.top);
    ctx.stroke();
  }

  return bytes;
}

// This function performs the following steps:
// 1. Capture the image from the camera stream (or from the local file).
// 2. Call the backend to detect the bounding box of the face in the image.
// 3. Call the backend to recognize the face. 
async function recognize(event) {
  event.preventDefault();
  hide("buttons");
  show("loader");
  message("Detecting face..");
  try {
    const [blob, scaling] = await capture_image();
    let result;
    result = await backend.detect(new Uint8Array(blob));
    if (!result.Ok) {
      throw result.Err.message;
    }
    let face = await render(scaling, result.Ok);
    message("Face detected. Recognizing..");
    result = await backend.recognize(new Uint8Array(face));
    if (!result.Ok) {
      throw result.Err.message;
    }
    let label = sanitize(result.Ok.label);
    let score = Math.round(result.Ok.score * 100) / 100;
    message(`${label}, difference=${score}`);
  } catch (err) {
    console.error(`An error occurred: ${err}`);
    message(err.toString());
  }
  hide("loader");
  show("restart");
  return false;
}

// This function performs the following steps:
// 1. Capture the image from the camera stream (or from the local file).
// 2. Call the backend to detect the bounding box of the face in the image.
// 3. Ask the user for their name.
// 4. Call the backend to store the image and the name of the user.
async function store(event) {
  event.preventDefault();
  hide("buttons");
  show("loader");
  message("Detecting face..");
  try {
    const [blob, scaling] = await capture_image();
    let result;
    result = await backend.detect(new Uint8Array(blob));
    if (!result.Ok) {
      throw result.Err.message;
    }
    let face = await render(scaling, result.Ok);
    message("Face detected. Adding..");
    let label = prompt("Enter name of the person");
    if (!label) {
      throw "cannot add without a name";
    }
    label = sanitize(label);
    message(`Face detected. Adding ${label}..`);
    result = await backend.add(label, new Uint8Array(face));
    if (!result.Ok) {
      throw result.Err.message;
    }
    message(`Successfully added ${label}.`);
  } catch (err) {
    console.error(`An error occurred: ${err}`);
    message("Failed to add the face: " + err.toString());
  }

  hide("loader");
  show("restart");
  return false;
}

// Invoked when a file is selected in the file input element. 
// Loads the given file as an image to show to the user.
async function load_local_image(event) {
  message("");
  let image = elem("image");
  try {
    const file = event.target.files[0];
    if (!file) {
      return false;
    }
    const url = await toDataURL(file);
    image.src = url;
  } catch (err) {
    message("Failed to select photo: " + err.toString());
  }
  hide("video");
  hide("canvas");
  show("image");
  show("buttons");
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

// Restarts the face recognition / addition user flow.
async function restart(event) {
  hide("restart");
  message("");
  if (video.srcObject) {
    event.preventDefault();
  }
  navigator.mediaDevices
    .getUserMedia({ video: true, audio: false })
    .then((stream) => {
      const video = elem("video");
      video.srcObject = stream;
      video.play();
      show("buttons");
    });
}

// Returns a DOM element by its id.
function elem(id) {
  return document.getElementById(id);
}

// Makes the given DOM element visible.
function show(id) {
  elem(id).className = "";
}

// Makes the given DOM element invisible.
function hide(id) {
  elem(id).className = "invisible";
}

// Sets the message element's text to the given text.
function message(m) {
  elem("message").innerText = m;
}

// Returns an PNG image from the canvas.
function serialize(canvas) {
  return new Promise((resolve) => canvas.toBlob((blob) => blob.arrayBuffer().then(resolve), "image/png", 0.9));
}

// Sanitizes the name string by filtering out characters that are not letters,
// numbers, spaces, and dashes.
function sanitize(name) {
  return name.match(/[\p{L}\p{N}\s_-]/gu).join('');
}
