import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */

const filesPath = (path) => `src/frontend/${path}`;

const config = {
	extensions: ['.svelte'],
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: [vitePreprocess()],
	
	kit: {
		adapter: adapter({
			fallback: 'index.html',
			precompress: false
		}),
		files: {
			assets: filesPath('static'),
			hooks: {
				client: filesPath('src/hooks.client'),
				server: filesPath('src/hooks.server')
			},
			lib: filesPath('src/lib'),
			params: filesPath('src/params'),
			routes: filesPath('src/routes'),
			serviceWorker: filesPath('src/service-worker'),
			appTemplate: filesPath('src/app.html'),
			errorTemplate: filesPath('src/error.html')
		}
	}
};
export default config;