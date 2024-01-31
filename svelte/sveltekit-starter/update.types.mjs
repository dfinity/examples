#!/usr/bin/env node

import { readdirSync } from 'fs';
import { readFile, writeFile } from 'fs/promises';
import { join } from 'path';

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
		await cleanTypes({});

		console.log(`Types declarations copied!`);
	} catch (err) {
		console.error(`Error while copying the types declarations.`, err);
	}
})();
