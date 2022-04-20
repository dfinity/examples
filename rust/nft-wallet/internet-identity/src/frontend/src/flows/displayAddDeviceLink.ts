import { html, render } from "lit-html";
import ClipboardJS from "clipboard";
import { checkmarkIcon } from "../components/icons";
import kjua from "kjua";

const pageContent = (link: string) => html`
  <style>
    .linkBox {
      display: flex;
    }
    #linkText {
      margin-right: 1rem;
      width: 80%;
    }
    #linkCopy {
      width: 20%;
    }
  </style>
  <div class="container">
    <h1>New device</h1>
    <p>
      Please open this URL on an already authenticated device. This page will
      automatically update when you have succeeded.
    </p>
    <label>URL</label>
    <div class="linkBox">
      <input id="linkText" value="${link}" readonly />
      <button id="linkCopy" data-clipboard-target="#linkText">Copy</button>
    </div>
    <button id="showQR">Display as QR Code</button>
  </div>
`;

const qrContent = (qrcode: Element) => html`
  <style>
    #qrBox {
      display: flex;
      justify-content: center;
      margin: 1rem 0;
    }
  </style>
  <div class="container">
    <h1>New device</h1>
    <p>
      Scan the QR Code below from your authenticated device. This page will
      automatically update when you have succeeded.
    </p>
    <label>QR Code</label>
    <div id="qrBox">${qrcode}</div>
    <button id="showURL">Display URL</button>
  </div>
`;

export const displayAddDeviceLink = (link: string): void => {
  const container = document.getElementById("pageContent") as HTMLElement;
  render(pageContent(link), container);
  init(link);
};

const displayAddDeviceQR = (link: string) => {
  const container = document.getElementById("pageContent") as HTMLElement;
  const el = kjua({ text: link, render: "svg" });
  render(qrContent(el), container);
  initQR(link);
};

const init = (link: string) => {
  const linkCopy = document.getElementById("linkCopy") as HTMLButtonElement;
  const showQR = document.getElementById("showQR") as HTMLButtonElement;

  new ClipboardJS(linkCopy).on("success", () => {
    const linkCopy = document.getElementById("linkCopy") as HTMLButtonElement;
    render(checkmarkIcon, linkCopy);
  });
  showQR.onclick = () => displayAddDeviceQR(link);
};

const initQR = (link: string) => {
  const showURL = document.getElementById("showURL") as HTMLButtonElement;
  showURL.onclick = () => displayAddDeviceLink(link);
};
