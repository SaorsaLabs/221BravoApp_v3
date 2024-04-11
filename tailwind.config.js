import { join } from 'path'

import forms from '@tailwindcss/forms';
import { skeleton } from '@skeletonlabs/tw-plugin'
import { bravoTheme } from './src/frontend/src/lib/theme/main_theme.js'

export default {
	darkMode: 'class',
	content: ['./src/frontend/src/**/*.{html,js,svelte,ts}', join(require.resolve('@skeletonlabs/skeleton'), '../**/*.{html,js,svelte,ts}')],
	theme: {
		extend: {
			minWidth: {
				'600': '600px',
			  }
		},
	},
	plugins: [
		skeleton({
			themes: {
				custom: [
					bravoTheme
				]
			}
		})
	]
};
