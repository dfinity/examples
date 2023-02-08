const fs = require("fs");
const path = require("path");

const network =
  process.env.DFX_NETWORK ||
  (process.env.NODE_ENV === "production" ? "ic" : "local");

const isNetworkLocal = network === "local";

function main(key, replacement) {
  const pathToSrcIndexHtml = path.join(
    __dirname,
    "..",
    "public",
    "index.src.html"
  );
  const content = fs.readFileSync(pathToSrcIndexHtml, { encoding: "utf8" });

  const replacementRegex = new RegExp(key, "g");
  const updatedContent = content.replace(replacementRegex, replacement);

  const pathToFinalIndexHtml = path.join(
    __dirname,
    "..",
    "public",
    "index.html"
  );
  fs.writeFileSync(pathToFinalIndexHtml, updatedContent);
}

const replacement = isNetworkLocal ? "localhost:8000" : "ic0.app";
main("ROLLUP_CONNECT_SRC_DOMAIN", replacement);
