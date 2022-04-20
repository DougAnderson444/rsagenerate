import adapter from '@sveltejs/adapter-auto';
import preprocess from 'svelte-preprocess';
import wasmPack from 'vite-plugin-wasm-pack';

import path from 'path';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: preprocess(),

	kit: {
		adapter: adapter(),
		vite: {
			plugins: [wasmPack('./wasm')],
			optimizeDeps: {
				exclude: ['./wasm']
			},
			resolve: {
				alias: {
					$components: path.resolve('./src/components')
				}
			}
		}
	}
};

export default config;
