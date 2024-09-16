// Update agent-js dependencies across all example projects.

import { $ } from "execa";
import { readdir, readFile, stat, writeFile } from "fs/promises";
import latestVersion from "latest-version";
import { join, resolve } from "path";

// Skip updating these projects.
const ignoreProjects = ["motoko/ic-pos"];

// Skip updating these packages.
const ignorePackages = [
  "@dfinity/internet-identity-vite-plugins",
  "@dfinity/internet-identity-vc-api",
];

const projectDirectories = [];
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
  let content = await readFile(filePath, "utf8");
  let updated = false;

  const updateMap = {};
  const packageRegex = /"(@dfinity\/[^"]+)":\s*"([^"]+)"/g;
  for (const match of content.matchAll(packageRegex)) {
    const pkg = match[1];
    const version = match[2];
    try {
      if (ignorePackages.includes(pkg)) {
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

// Update the `package-lock.json` file in the given directory.
async function updatePackageLock(directory) {
  const packageJsonPath = join(directory, "package.json");
  try {
    await $({
      stdio: "inherit",
      cwd: directory,
    })`npm install --package-lock-only`;
    console.log(`Updated lockfile for ${packageJsonPath}`);
  } catch (err) {
    console.error(err);
    throw new Error(`Error while updating lockfile for ${packageJsonPath}`);
  }
}

// Search for and update directories with a `package.json` file.
// Returns a list of directories containing npm projects.
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
        if (
          ignoreProjects.some((path) => resolve(directory) === resolve(path))
        ) {
          return;
        }
        projectDirectories.push(directory);
        await updatePackageJson(filePath);
      }
    })
  );
}

(async () => {
  await searchAndUpdate(process.cwd());
  for (const directory of projectDirectories) {
    await updatePackageLock(directory);
  }
  console.log("Completed");
})();
