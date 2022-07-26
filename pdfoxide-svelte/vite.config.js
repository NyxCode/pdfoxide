import { sveltekit } from '@sveltejs/kit/vite';
import wasmPack from 'vite-plugin-wasm-pack';
import wasm from "vite-plugin-wasm";

/** @type {import('vite').UserConfig} */
const config = {
	plugins: [wasm(), sveltekit(), {
		name: "configure-response-headers",
		configureServer: (server) => {
			server.middlewares.use((_req, res, next) => {
				res.setHeader("Cross-Origin-Embedder-Policy", "require-corp");
				res.setHeader("Cross-Origin-Opener-Policy", "same-origin");
				next();
			});
		},
	},],
	server: {
		fs: {
			allow: [".."]
		},
		headers: {
			'Cross-Origin-Opener-Policy': 'same-origin',
			'Origin-Embedder-Policy': 'require-corp'
		},
	},
	
};

export default config;
