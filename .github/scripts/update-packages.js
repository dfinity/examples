// Update agent-js dependencies across all example projects.

import { readFile, writeFile, readdir, stat } from "fs/promises";
import { resolve, join } from "path";
import latestVersion from "latest-version";

// Skip updating these packages.
const ignoreList = [
  "@dfinity/internet-identity-vite-plugins",
  "@dfinity/internet-identity-vc-api",
];

const visitedPackageJsonFiles = [];
const versionPromiseMap = new Map();

// Find and cache the latest version of a package.
async function findLatestVersion(pkg) {
  if (versionPromiseMap.has(pkg)) {
    return versionPromiseMap.get(pkg);
  }
  const version = latestVersion(pkg);
  versionPromiseMap.set(pkg, version);
  return version;
}

// Update all `package.json` files in the repository.
async function updatePackageJson(filePath) {
  filePath = resolve(filePath);
  if (visitedPackageJsonFiles.includes(filePath)) {
    console.log("Already visited package.json file:", filePath);
    return;
  }
  visitedPackageJsonFiles.push(filePath);

  let content = await readFile(filePath, "utf8");
  let updated = false;

  const updateMap = {};
  const packageRegex = /"(@dfinity\/[^"]+)":\s*"([^"]+)"/g;
  for (const match of content.matchAll(packageRegex)) {
    const pkg = match[1];
    const version = match[2];
    try {
      if (ignoreList.includes(pkg)) {
        continue;
      }
      const newVersion = `^${await findLatestVersion(pkg)}`;
      if (version !== newVersion) {
        const regex = new RegExp(`("${pkg}"\\s*:\\s*")([^"]+)`, "g");
        if (!regex.test(content)) {
          throw new Error(
            "Bug: missing regex pattern to replace package version"
          );
        }
        content = content.replace(regex, `$1${newVersion}`);
        updated = true;
        updateMap[pkg] = newVersion;
      }
    } catch (error) {
      console.error(`Error while updating ${pkg}:`);
      throw error;
    }
  }

  if (updated) {
    await writeFile(filePath, content);
    console.log(`Updated ${filePath}:`, updateMap);
  }
}

// Recursively search for and update directories with a `package.json` file.
async function searchAndUpdate(directory) {
  const files = await readdir(directory);
  await Promise.all(
    files.map(async (filename) => {
      const filePath = join(directory, filename);
      if ((await stat(filePath)).isDirectory()) {
        if (filename === "node_modules") {
          return;
        }
        await searchAndUpdate(filePath);
      } else if (filename === "package.json") {
        await updatePackageJson(filePath);
      }
    })
  );
}

searchAndUpdate(process.cwd()).then(() => console.log("Completed"));
