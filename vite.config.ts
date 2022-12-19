import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';

const config: UserConfig = {
	server: {
		proxy: {}
	},
	plugins: [sveltekit()]
};

export default config;
