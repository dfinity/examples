import { html } from "lit-html";

export const compatibilityData = {
  note: "Browser support for WebAuthentication is constantly evolving. Your preferred browser may not support WebAuthentication, or may only support it using a security key. If you run into issues, please try again with one of our recommended browsers.",

  desktop: [
    "Chrome version 67 or newer",
    "Firefox version 60 or newer",
    "Safari version 13 or newer",
    "Edge version 18 or newer",
  ],

  mobile: [
    "Chrome (Android)",
    "Safari 14.4 or newer (iOS)",
    "Firefox latest (iOS)",
    "Chrome latest (iOS)",
  ],
};

export const compatibilityChart = html`<p>${compatibilityData.note}</p>
  <h3>For Desktop</h3>
  <ul>
    ${compatibilityData.desktop.map((browser) => html`<li>${browser}</li>`)}
  </ul>
  <h3>For Mobile</h3>
  <ul>
    ${compatibilityData.mobile.map((browser) => html`<li>${browser}</li>`)}
  </ul>`;
