import type { PageServerLoad } from './$types';
import { error, redirect } from '@sveltejs/kit';
import { PUBLIC_BACKEND } from '$env/static/public';
import * as api from '$lib/client/api';

export const prerender = false;

export const load: PageServerLoad = async ({ cookies, fetch }) => {
	const auth = cookies.get('access_token');
	console.log(auth);

	if (!auth || auth == '') {
		throw redirect(307, `${PUBLIC_BACKEND}/auth/login`);
		// throw error(401, { message: 'Unauthorized' });
	}

	let user: api.User;

	try {
		user = await api.getUser({ fetch, token: auth });
	} catch (ex) {
		if (ex instanceof api.ApiError) {
			console.error(ex);
			if (ex.code == 243) {
				throw redirect(307, `${PUBLIC_BACKEND}/auth/logout`);
			}
		}

		console.error(ex);

		throw error(500, {
			message: 'Internal Server Error'
		});
	}

	if (!user.is_admin) {
		throw redirect(307, '/participate');
	}

	return {
		user: user,
		teams: await api.getTeams({ fetch, token: auth })
	};
};
