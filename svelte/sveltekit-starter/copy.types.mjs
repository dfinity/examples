#!/usr/bin/env node

import { existsSync, lstatSync, mkdirSync, readdirSync } from 'fs';
import { copyFile, readFile, writeFile } from 'fs/promises';
import { basename, dirname, extname, join } from 'path';

const copyTypes = async ({ src = `.dfx/local/canisters/`, dest = `./src/declarations` }) => {
	const promises = readdirSync(src)
		.filter((sub) => !['idl'].includes(sub))
		.map(
			(dir) =>
				new Promise(async (resolve) => {
					const [srcPath, destPath] = [src, dest].map((dirPath) => join(dirPath, dir));

					const stat = lstatSync(srcPath);

					if (stat.isDirectory()) {
						await copyTypes({ src: srcPath, dest: destPath });
					} else if (stat.isFile()) {
						await copyDeclarationsFile({ srcPath, destPath });
					}

					resolve();
				})
		);

	await Promise.all(promises);
};

const copyDeclarationsFile = async ({ srcPath, destPath }) => {
	if (!['.did', '.ts', '.js'].includes(extname(srcPath))) {
		return;
	}

	if (extname(srcPath) === '.mjs') {
		return;
	}

	if (basename(srcPath).includes('.old')) {
		return;
	}

	// Create destination directory if does not exists
	const dir = dirname(destPath);
	if (!existsSync(dir)) {
		mkdirSync(dir, { recursive: true });
	}

	await copyFile(srcPath, destPath);
};

/**
 * We have to manipulate the types as long as https://github.com/dfinity/sdk/discussions/2761 is not implemented
 */
const cleanTypes = async ({ dest = `./src/declarations` }) => {
	const promises = readdirSync(dest).map(
		(dir) =>
			new Promise(async (resolve) => {
				const indexPath = join(dest, dir, 'index.js');

				const content = await readFile(indexPath, 'utf-8');
				const clean = content
					.replace(/export const \w* = createActor\(canisterId\);/g, '')
					.replace(/export const canisterId = process\.env\.\w*_CANISTER_ID;/g, '');

				await writeFile(indexPath, clean, 'utf-8');

				resolve();
			})
	);

	await Promise.all(promises);
};

(async () => {
	try {
		await copyTypes({});
		await cleanTypes({});

		console.log(`Types declarations copied!`);
	} catch (err) {
		console.error(`Error while copying the types declarations.`, err);
	}
})();
