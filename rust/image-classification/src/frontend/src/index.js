import { backend } from "../../declarations/backend";

function elem(id) {
  return document.getElementById(id);
}

function show(id) {
  elem(id).className = "";
}

function hide(id) {
  elem(id).className = "invisible";
}

function message(m) {
  elem("message").innerText = m;
}

function serialize(canvas) {
  return new Promise((resolve) => canvas.toBlob((blob) => blob.arrayBuffer().then(resolve), "image/png", 0.9));
}

function sanitize(name) {
  return name.match(/[\p{L}\p{N}\s_-]/gu).join('');
}

window.onload = async () => {
  elem("recognize").onclick = recognize;
  elem("store").onclick = store;
  elem("file").onchange = upload;
  elem("canvas").onclick = restart;
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

async function render(scaling, box) {
  box.left = Math.round((box.left * 320 - scaling.x) / scaling.scale);
  box.right = Math.round((box.right * 320 - scaling.x) / scaling.scale);
  box.top = Math.round((box.top * 240 - scaling.y) / scaling.scale);
  box.bottom = Math.round((box.bottom * 240 - scaling.y) / scaling.scale);

  const canvas = elem("canvas");

  const small = document.createElement("canvas");
  small.width = 140;
  small.height = 140;
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
    message(`${label}, score=${score}`);
  } catch (err) {
    console.error(`An error occurred: ${err}`);
    message(err.toString());
  }
  hide("loader");
  return false;
}

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
  return false;
}

async function upload(event) {
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

async function restart(event) {
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