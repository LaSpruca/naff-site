import type { PageServerLoad } from './$types';
import { error, redirect } from '@sveltejs/kit';
import { PUBLIC_BACKEND } from '$env/static/public';
import * as api from '$lib/client/api';

export const prerender = false;

export const load: PageServerLoad = async ({ request, cookies, fetch }) => {
	const auth = cookies.get('access_token');

	if (!auth || auth == '') {
		throw redirect(307, `${PUBLIC_BACKEND}/auth/login`);
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
		if (ex instanceof api.ApiError) {
			if (ex.code == 243) {
				throw redirect(307, `${PUBLIC_BACKEND}/auth/logout`);
			}
		}

		throw error(500, {
			message: 'Internal Server Error'
		});
	}

	return {
		user,
		team,
		members
	};
};

export const _load: PageServerLoad = async ({ request, cookies, fetch }) => {
	const auth = cookies.get('access_token');

	if (!auth) {
		throw redirect(307, `${PUBLIC_BACKEND}/auth/login`);
	} else {
		try {
			const userData = await fetch(`${PUBLIC_BACKEND}/api/user`, {
				headers: {
					Authorization: `Bearer ${auth}`
				}
			});

			if (userData.status == 401) {
				throw error(401, {
					message: 'You are not currently logged in, please clear cookies and try again'
				});
			} else if (userData.status != 200) {
				throw error(500, { message: 'Internal server error' });
			}

			const team = await fetch(`${PUBLIC_BACKEND}/api/team`, {
				headers: {
					Authorization: `Bearer ${auth}`
				}
			});

			const teamData = (await team.json()) as api.Team;

			let membersData = [];

			if (team.status == 200) {
				const members = await fetch(`${PUBLIC_BACKEND}/api/team/${teamData.id}/members`);
			}
			return {
				userData: (await userData.json()) as api.User,
				teamData: team.status == 200 ? ((await team.json()) as api.Team) : null
			};
		} catch (ex) {
			console.log(ex);
			throw error(500, { message: "Couldn't fetch user data" });
		}
	}
};
