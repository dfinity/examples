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
  navigator.mediaDevices
    .getUserMedia({ video: true, audio: false })
    .then((stream) => {
      const video = elem("video");
      video.srcObject = stream;
      video.play();
      const logo = elem("logo");
      show("buttons");
      show("video");
    })
    .catch((err) => {
      const video = elem("video");
      console.error(`An error occurred: ${err}`);
      message("Failed to start camera: " + err.toString());
    });
}

async function capture_image() {
  const video = elem("video");
  const canvas = elem("canvas");
  canvas.width = video.videoWidth;
  canvas.height = video.videoHeight;
  const context = canvas.getContext("2d");
  context.drawImage(video, 0, 0, video.videoWidth, video.videoHeight);

  const resized = document.createElement("canvas");
  resized.width = 320;
  resized.height = 240;
  let scale = Math.min(resized.width / canvas.width, resized.height / canvas.height);
  let width = canvas.width * scale;
  let height = canvas.height * scale;
  let x = resized.width / 2 - width / 2;
  let y = resized.height / 2 - height / 2;
  const ctx = resized.getContext("2d");
  if (ctx) {
    ctx.drawImage(canvas, x, y, width, height);
  }
  let bytes = await serialize(canvas);

  video.srcObject.getTracks().forEach((track) => track.stop());

  hide("video");
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
    result = await backend.detect_query(new Uint8Array(blob));
    if (!result.Ok) {
      throw JSON.stringify(result.Err);
    }
    let face = await render(scaling, result.Ok);
    message("Face detected. Recognizing..");
    result = await backend.recognize(new Uint8Array(face));
    if (!result.Ok) {
      throw JSON.stringify(result.Err);
    }
    let label = sanitize(result.Ok.label);
    let score = Math.round(result.Ok.score * 100) / 100;
    message(`Recognized ${label} with score=${score}`);
  } catch (err) {
    console.error(`An error occurred: ${err}`);
    message("Failed to detect the face: " + err.toString());
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
    result = await backend.detect_query(new Uint8Array(blob));
    if (!result.Ok) {
      throw JSON.stringify(result.Err);
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
      throw JSON.stringify(result.Err);
    }
    message(`Successfully added ${label}.`);
  } catch (err) {
    console.error(`An error occurred: ${err}`);
    message("Failed to add the face: " + err.toString());
  }

  hide("loader");
  return false;
}
