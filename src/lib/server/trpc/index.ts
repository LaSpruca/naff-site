import type { inferAsyncReturnType } from '@trpc/server';
import * as trpc from '@trpc/server';

export const createContext = () => {
	return {};
};

export const router = trpc.router<inferAsyncReturnType<typeof createContext>>().query('hello', {
	resolve: () => 'world'
});

export type Router = typeof router;
