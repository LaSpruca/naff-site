import type { User } from '@auth0/auth0-spa-js';
import { derived, writable } from 'svelte/store';

export const token = writable<String | null>(null);
export const user = writable<User | null>(null);
export const loggedIn = derived(token, (token) => token !== null);
export const auth0Ready = writable(false);
