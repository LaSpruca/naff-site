import { browser } from '$app/environment';
import { Auth0Client } from '@auth0/auth0-spa-js';
import { token, user } from './stores';

const client = new Auth0Client({
	clientId: 'vuhclMdObJUN8N82Edx6k8wUbLp15MFN',
	domain: 'naff-dev.au.auth0.com'
});

// Function to check if the user is signed in
const updateLoginStatus = async () => {
	if (await client.isAuthenticated()) {
		user.set((await client.getUser()) ?? null);
		token.set(await client.getTokenSilently());
	} else {
		user.set(null);
		token.set(null);
	}
};

// If in the browser, check for any existing user sessions
if (browser) {
	await client.checkSession();
	await updateLoginStatus();
}

export const login = async () => {
	await client.loginWithPopup();
	updateLoginStatus();
};

export const logout = async () => {
	await client.logout();
	updateLoginStatus();
};
