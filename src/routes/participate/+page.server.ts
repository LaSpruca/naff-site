import type { PageServerLoad } from './$types';
import { error, redirect } from '@sveltejs/kit';
import { PUBLIC_BACKEND } from '$env/static/public';
import * as api from '$lib/client/api';

export const prerender = false;

export const load: PageServerLoad = async ({ cookies, fetch }) => {
	const auth = cookies.get('access_token');

	if (!auth || auth == '') {
		throw redirect(307, `${PUBLIC_BACKEND}/auth/login`);
		// throw error(401, { message: 'Unauthorized' });
	}

	let user;
	let team;
	let members;

	try {
		user = await api.getUser({ fetch, token: auth });
		team = await api.getTeam({ fetch, token: auth });

		if (team) {
			members = await api.getMembers(team.id, { fetch, token: auth });
		}
	} catch (ex) {
		console.log(ex);
		if (ex instanceof api.ApiError) {
			if (ex.code == 243) {
				throw redirect(307, `${PUBLIC_BACKEND}/auth/logout`);
			}
		}

		console.error(ex);

		throw error(500, {
			message: 'Internal Server Error'
		});
	}

	if (user.is_admin) {
		throw redirect(307, '/admin');
	}

	return {
		user,
		team,
		members
	};
};
