import type { Handle } from '@sveltejs/kit';
import { createContext, router } from '$lib/server/trpc';
import { createTRPCHandle } from 'trpc-sveltekit';

export const handle: Handle = async ({ event, resolve }) => {
	const response = await createTRPCHandle({
		url: '/trpc', // optional; defaults to '/trpc'
		router,
		createContext, // optional
		event,
		resolve
	});

	return response;
};
